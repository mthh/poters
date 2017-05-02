#![crate_name="poters"]

extern crate csv;
extern crate geojson;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate error_chain;

mod utils;
mod compute;
mod errors;
pub use self::utils::{
    parse_csv_points, parse_json_points, parse_geojson_points,
    save_json_points, save_geojson_points, ValuesJson};
pub use self::compute::{FuncNames, Config, Bbox, PtValue, smooth};
pub use errors::*;

#[cfg(test)]
mod test;
