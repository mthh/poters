use rustc_serialize::json;
use std::f64;
use std::fs::File;
use std::io::{Read,Write};
use errors::*;

const PI:f64 = f64::consts::PI;

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
    pub fn new(range: f64, smoothing_fun: FuncNames) -> Config {
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
    pub fn new(min_lat:f64, max_lat:f64, min_lon:f64, max_lon:f64) -> Bbox {
        Bbox {min_lat: min_lat, max_lat: max_lat, min_lon: min_lon, max_lon: max_lon}
    }
}
#[derive(Debug,Clone)]
#[derive(RustcDecodable,RustcEncodable)]
pub struct PtValue {
    lat: f64,
    lon: f64,
    value: f64
}

#[derive(RustcDecodable,RustcEncodable)]
pub struct ValuesJson  {
    values: Vec<PtValue>
}

impl PtValue {
    #[inline(always)]
    pub fn new(lat: f64, lon: f64, value: f64) -> PtValue {
        PtValue {lat: lat, lon: lon, value: value}
    }
}

#[inline(always)]
fn amortized_disk(pot: f64, dst: f64, range: f64) -> f64 {
    pot / (1.0 + dst)
}

#[inline(always)]
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

pub fn parse_json_points(path: &str) -> Result<Vec<PtValue>> {
    let mut file = File::open(path)?;
    let mut raw_json = String::new();
    file.read_to_string(&mut raw_json)?;
    let decoded: ValuesJson = json::decode(&raw_json)?;
    let mut res = Vec::with_capacity(decoded.values.len());
    for elem in decoded.values.iter(){
        res.push(PtValue::new(elem.lat * PI / 180.0, elem.lon * PI / 180.0, elem.value));
    }
    Ok(res)
}

pub fn save_json_points(path: &str, obs_points: Vec<Vec<PtValue>>) -> Result<()> {
    let mut res = Vec::new();
    for arr in obs_points.iter(){
        for elem in arr.iter(){
            res.push(elem);
        }
    }
    let encoded = json::encode(&res)?;
    let mut file = File::create(path)?;
    file.write(encoded.as_bytes())?;
    Ok(())
}

pub fn smooth(reso_lat: u32, reso_lon: u32, bbox: Bbox, obs_points: &mut [PtValue], configuration: Config) -> Result<Vec<Vec<PtValue>>> {
    let mut plots = Vec::with_capacity(reso_lat as usize);
    for _ in 0..reso_lat {
        plots.push(vec![PtValue{lat: 0.0, lon: 0.0, value: 0.0}; reso_lon as usize]);
    }
    let mut lon_step = (bbox.max_lon - bbox.min_lon) / reso_lon as f64;
    let mut lat_step = (bbox.max_lat - bbox.min_lat) / reso_lat as f64;
    let range = configuration.fparam;
    for i in 0..reso_lat as usize {
        for j in 0..reso_lon as usize {
            plots[i][j] = PtValue {
                lat: bbox.min_lat + lat_step * i as f64,
                lon: bbox.min_lon + lon_step * j as f64,
                value: 0.0
            };
        }
    }
	lon_step *= PI / 180.0;
	lat_step *= PI / 180.0;
    do_smooth(&bbox, lon_step, lat_step, range, reso_lon, reso_lat,
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
    let bbox_lon_min = bbox.min_lon * PI / 180.0;
    let bbox_lat_min = bbox.min_lat * PI / 180.0;
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

			let deltalon = (((range / 6368.0).cos() - clat.sin().powi(2)) / clat.cos().powi(2)).acos();
			let lonmax = clon + deltalon;
			let lonmin = clon - deltalon;

			let _jmin = ((lonmin - bbox_lon_min) / lon_step).floor() as i32;
			let mut jmax = 1 + ((lonmax - bbox_lon_min) / lon_step).ceil() as u32;
            let jmin = if _jmin < 0 { 0 } else { _jmin as u32 };
			if jmax > lon_range {
                jmax = lon_range;
            }

            let mut contrib = Vec::with_capacity((imax as i32 - imin as i32 + 1) as usize);
            for _ in 0..(imax as i32 - imin as i32 + 1) {
                contrib.push(vec![0.0; (jmax - jmin + 1) as usize]);
            }

            for i in imin..imax as i32 {
                for j in jmin..jmax {
                    let tmp = 6368.0 * ((bbox_lat_min + lat_step * i as f64).cos() * clat.cos() *
                            ((bbox_lon_min + lon_step * j as f64).cos() * clon.cos() + (bbox_lon_min + lon_step * j as f64).sin() * clon.sin())
                            + (bbox_lat_min + lat_step * i as f64).sin() * clat.sin()).acos();
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
