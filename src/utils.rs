use rustc_serialize::json;
use std::f64;
use std::fs::File;
use std::io::{Read,Write};
use csv;
use errors::*;
use compute::PtValue;

const PI:f64 = f64::consts::PI;

#[derive(RustcDecodable,RustcEncodable)]
pub struct ValuesJson  {
    values: Vec<PtValue>
}

pub fn parse_json_points(path: &str) -> Result<Vec<PtValue>> {
    let mut file = File::open(path)?;
    let mut raw_json = String::new();
    file.read_to_string(&mut raw_json)?;
    let decoded: ValuesJson = json::decode(&raw_json)?;
    let mut res = Vec::with_capacity(decoded.values.len());
    for elem in decoded.values.iter(){
        let (lat, lon, val) = elem.get_triplet();
        res.push(PtValue::new(lat * PI / 180.0, lon * PI / 180.0, val));
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

pub fn parse_csv_points(path: &str) -> Result<Vec<PtValue>> {
    let mut rdr = csv::Reader::from_file(path)?;
    let mut res = Vec::new();
    for record in rdr.decode() {
        let (lat, lon, val): (f64, f64, f64) = record?;
        res.push(PtValue::new(lat * PI / 180.0, lon * PI / 180.0, val));
    }
    Ok(res)
}
