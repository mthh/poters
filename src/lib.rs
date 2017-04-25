#![crate_name="poters"]

extern crate csv;
extern crate geojson;
extern crate rustc_serialize;

#[macro_use]
extern crate error_chain;

mod utils;
mod compute;
mod errors;
pub use self::utils::{parse_csv_points, parse_json_points, parse_geojson_points, save_json_points, ValuesJson};
pub use self::compute::{FuncNames, Config, Bbox, PtValue, smooth};

// use errors::*;

#[cfg(test)]
mod test;
