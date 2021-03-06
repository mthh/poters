use errors::*;
use std::f64;

static PI:f64 = f64::consts::PI;
static DEG2RAD: f64 = f64::consts::PI / 180.0;

#[derive(Debug)]
pub enum FuncNames {
  Disk = 0,
  AmortizedDisk = 1,
  Gaussian = 2,
  Exponential = 3,
  Pareto = 4
}

#[derive(Debug)]
pub struct Config {
    smoothing_fun_t: FuncNames,
    fparam: f64,
}

impl Config {
    pub fn new(range: f64, smoothing_fun: FuncNames) -> Self {
        Config { fparam: range, smoothing_fun_t: smoothing_fun }
    }
}

#[derive(Debug)]
pub struct Bbox {
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64
}

impl Bbox {
    pub fn new(min_lat:f64, max_lat:f64, min_lon:f64, max_lon:f64) -> Self {
        Bbox {min_lat: min_lat, max_lat: max_lat, min_lon: min_lon, max_lon: max_lon}
    }
}

#[derive(Debug,Clone)]
#[derive(Serialize,Deserialize)]
pub struct PtValue {
    lat: f64,
    lon: f64,
    value: f64
}

impl PtValue {
    #[inline(always)]
    pub fn new(lat: f64, lon: f64, value: f64) -> Self {
        PtValue {lat: lat, lon: lon, value: value}
    }
    pub fn get_triplet(&self) -> (f64, f64, f64) {
        (self.lat, self.lon, self.value)
    }
}

#[inline(always)]
#[allow(unused_variables)]
fn amortized_disk(pot: f64, dst: f64, range: f64) -> f64 {
    pot / (1.0 + dst)
}

#[inline(always)]
#[allow(unused_variables)]
fn disk(pot: f64, dst: f64, range: f64) -> f64 {
    pot
}

#[inline(always)]
fn gaussian(pot: f64, dst: f64, range: f64) -> f64 {
    pot * ((-(PI / (4.0 * range.powi(2)) ) * dst.powi(2)).exp())
}

#[inline(always)]
fn exponential(pot: f64, dst: f64, range: f64) -> f64 {
    pot * ((-(2.0 / range) * (dst)).exp())
}

#[inline(always)]
fn paretopot(pot: f64, dst: f64, range: f64) -> f64 {
    let tmp = dst.powi(2);
    pot * (1.0 / (1.0 + (2.0 / range * tmp.powi(2) )))
}


pub fn smooth(reso_lat: u32, reso_lon: u32, bbox: &Bbox, obs_points: &[PtValue], configuration: Config) -> Result<Vec<Vec<PtValue>>> {
    let mut lon_step = (bbox.max_lon - bbox.min_lon) / reso_lon as f64;
    let mut lat_step = (bbox.max_lat - bbox.min_lat) / reso_lat as f64;
    let range = configuration.fparam;
    let mut plots = Vec::with_capacity(reso_lat as usize);
    for i in 0..plots.capacity() {
        let mut ivec = Vec::with_capacity(reso_lon as usize);
        for j in 0..ivec.capacity() {
            ivec.push(PtValue{lat: bbox.min_lat + lat_step * i as f64, lon: bbox.min_lon + lon_step * j as f64, value: 0.0})
        }
        plots.push(ivec)
    }
	lon_step *= DEG2RAD;
	lat_step *= DEG2RAD;
    do_smooth(bbox, lon_step, lat_step, range, reso_lon, reso_lat,
              &mut plots, obs_points, configuration.smoothing_fun_t);
    Ok(plots)
}


fn do_smooth(bbox: &Bbox, lon_step: f64, lat_step: f64, range: f64, lon_range: u32, lat_range: u32,
             plots: &mut Vec<Vec<PtValue>>, obs_points: &[PtValue], func: FuncNames){
    let smooth_func = match func {
        FuncNames::Disk => disk,
        FuncNames::AmortizedDisk => amortized_disk,
        FuncNames::Gaussian => gaussian,
        FuncNames::Exponential => exponential,
        FuncNames::Pareto => paretopot
    };
    let bbox_lon_min = bbox.min_lon * DEG2RAD;
    let bbox_lat_min = bbox.min_lat * DEG2RAD;
    let nb = obs_points.len();
    let mut obs_pt_sum = 0.0;
    let mut total_sum = 0.0;

    for k in 0..nb {
        let pot = obs_points[k].value;
        obs_pt_sum = obs_pt_sum + pot;

        if pot > 0.0 {
            let clat = obs_points[k].lat;
            let clon = obs_points[k].lon;
            let mut sum = 0.0;
            let latmax = (clat.cos() * (range / 6368.0).cos() - (clat.sin() * (range / 6368.).sin()).abs()).acos();
			let mut latmin = (clat.cos() * (range / 6368.0).cos() + (clat.sin() * (range / 6368.).sin()).abs()).acos();
			if latmin > clat {
                latmin = 2.0 * clat - latmin;
            }

            let mut imin = ((latmin - bbox_lat_min) / lat_step).floor() as i32;
			let mut imax = (1.0 + ((latmax - bbox_lat_min) / lat_step).ceil()) as u32;

			if imin < 0 { imin = 0; }
			if imax > lat_range { imax = lat_range; }

			let deltalon = (((range / 6368.0).cos() - (clat.sin()).powi(2)) / (clat.cos()).powi(2)).acos();
			let lonmax = clon + deltalon;
			let lonmin = clon - deltalon;

			let _jmin = ((lonmin - bbox_lon_min) / lon_step).floor() as i32;
			let mut jmax = 1 + ((lonmax - bbox_lon_min) / lon_step).ceil() as u32;
            let jmin = if _jmin < 0 { 0 } else { _jmin as u32 };
			if jmax > lon_range {
                jmax = lon_range;
            }

            let mut contrib = Vec::with_capacity((imax as i32 - imin as i32) as usize);
            for _ in 0..(imax as i32 - imin as i32) {
                contrib.push(vec![0.0; (jmax - jmin) as usize]);
            }
            for i in imin..imax as i32 {
                for j in jmin..jmax {
                    let ii = i as f64;
                    let jj = j as f64;
                    let tmp = 6368.0 * ((bbox_lat_min + lat_step * ii).cos() * clat.cos() * ( (bbox_lon_min + lon_step * jj).cos() * clon.cos() + (bbox_lon_min + lon_step * jj).sin() * clon.sin()) + (bbox_lat_min + lat_step * ii).sin() *clat.sin()).acos();

                    if tmp < range {
                        contrib[(i - imin) as usize][(j - jmin) as usize] = smooth_func(pot, tmp, range);
                        sum += contrib[(i - imin) as usize][(j - jmin) as usize];
                    } else {
                        contrib[(i - imin) as usize][(j - jmin) as usize] = 0.0;
                    }
                }
            }

            if sum > 0.0 {
                for i in imin..imax as i32 {
                    for j in jmin..jmax {
                        let c = contrib[(i - imin) as usize][(j - jmin) as usize];
                        if c > 0.0 {
                            plots[i as usize][j as usize].value += c * pot / sum;
                            total_sum = total_sum + (c * pot / sum);
                        }
                    }
                }
            }
        }
    }
}
