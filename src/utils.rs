use rustc_serialize::json;
use std::f64;
use std::fs::File;
use std::io::{Read,Write};
use csv;
use geojson;
use errors::*;
use compute::PtValue;

static DEG2RAD: f64 = f64::consts::PI / 180.0;


#[derive(RustcDecodable,RustcEncodable)]
pub struct ValuesJson  {
    values: Vec<PtValue>
}

pub fn parse_geojson_points(path: &str, field_name: &str) -> Result<Vec<PtValue>> {
    let mut file = File::open(path)?;
    let mut raw_json = String::new();
    file.read_to_string(&mut raw_json)?;
    let decoded_geojson = raw_json.parse::<geojson::GeoJson>()?;
    let features = match decoded_geojson {
        geojson::GeoJson::FeatureCollection(collection) => collection.features,
        _ => return Err("Error: Expected a FeatureCollection".into())
    };
    let mut res = Vec::new();
    for ft in features {
        if let Some(ref geometry) = ft.geometry {
            if let ::geojson::Value::Point(ref positions) = geometry.value {
                let prop = ft.properties.unwrap();
                let value = prop.get(field_name).unwrap();
                let val = if value.is_number() {
                    value.as_f64().unwrap()
                } else {
                    value.to_string().replace("\"", "").parse::<f64>()?
                };
                res.push(PtValue::new(positions[1] * DEG2RAD, positions[0] * DEG2RAD, val));
            }
        }
    }
    Ok(res)
}

pub fn parse_json_points(path: &str) -> Result<Vec<PtValue>> {
    let mut file = File::open(path)?;
    let mut raw_json = String::new();
    file.read_to_string(&mut raw_json)?;
    let decoded: ValuesJson = json::decode(&raw_json)?;
    let mut res = Vec::with_capacity(decoded.values.len());
    for elem in decoded.values.iter(){
        let (lat, lon, val) = elem.get_triplet();
        res.push(PtValue::new(lat * DEG2RAD, lon * DEG2RAD, val));
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
        res.push(PtValue::new(lat * DEG2RAD, lon * DEG2RAD, val));
    }
    Ok(res)
}
