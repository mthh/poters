#![crate_name="poters"]

extern crate csv;
extern crate rustc_serialize;

#[macro_use]
extern crate error_chain;

mod utils;
mod compute;
pub mod errors;
pub use self::utils::{parse_csv_points, parse_json_points, save_json_points, ValuesJson};
pub use self::compute::{FuncNames, Config, Bbox, PtValue, smooth};
// pub use self::compute::{FuncNames, Config, Bbox, PtValue, ValuesJson, parse_json_points, parse_csv_points, save_json_points, smooth};
use errors::*;

#[cfg(test)]
mod test;
