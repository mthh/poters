use serde_json;
use std::f64;
use std::fs::File;
use std::io::{Read,Write};
use csv;
use geojson;
use errors::*;
use compute::PtValue;

static DEG2RAD: f64 = f64::consts::PI / 180.0;


#[derive(Serialize, Deserialize)]
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
        _ => return Err("Error: expected a FeatureCollection".into())
    };
    let mut res = Vec::with_capacity(features.len());
    for ft in features {
        if let Some(ref geometry) = ft.geometry {
            if let ::geojson::Value::Point(ref positions) = geometry.value {
                let prop = ft.properties.unwrap();
                let value = prop.get(field_name).unwrap();
                let val = match *value {
                    serde_json::Value::Number(ref v) => v.as_f64().unwrap(),
                    serde_json::Value::String(ref v) => v.to_string().parse::<f64>()?,
                    _ => return Err("Invalid datastructure".into())
                };
                res.push(PtValue::new(positions[1] * DEG2RAD, positions[0] * DEG2RAD, val));
            }
        } else {
            return Err("Error: empty FeatureCollection".into());
        }
    }
    Ok(res)
}

pub fn parse_json_points(path: &str) -> Result<Vec<PtValue>> {
    let mut file = File::open(path)?;
    let mut raw_json = String::new();
    file.read_to_string(&mut raw_json)?;
    let decoded: serde_json::Value = serde_json::from_str(&raw_json)?;
    let ref arr = if decoded.is_object() && !decoded.get("values").is_none() && decoded["values"].is_array() {
        decoded["values"].as_array().unwrap()
    } else if decoded.is_array() {
        decoded.as_array().unwrap()
    } else {
        return Err("Invalid datastructure".into());
    };
    let mut res = Vec::with_capacity(arr.len());
    for elem in arr.iter() {
        let value = match elem["value"] {
            serde_json::Value::Number(ref val) => val.as_f64().unwrap(),
            serde_json::Value::String(ref val) => val.to_string().parse::<f64>()?,
            _ => return Err("Invalid datastructure".into())
        };
        let lat = match elem["lat"] {
            serde_json::Value::Number(ref val) => val.as_f64().unwrap(),
            serde_json::Value::String(ref val) => val.to_string().parse::<f64>()?,
            _ => return Err("Invalid datastructure".into())
        };
        let lon = match elem["lon"] {
            serde_json::Value::Number(ref val) => val.as_f64().unwrap(),
            serde_json::Value::String(ref val) => val.to_string().parse::<f64>()?,
            _ => return Err("Invalid datastructure".into())
        };
        res.push(PtValue::new(lat * DEG2RAD, lon * DEG2RAD, value));
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
    let encoded = serde_json::to_string(&res)?;
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
