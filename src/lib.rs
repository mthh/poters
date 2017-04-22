#![crate_name="poters"]

extern crate rustc_serialize;

#[macro_use]
extern crate error_chain;

mod compute;
pub mod errors;
pub use self::compute::{FuncNames, Config, Bbox, PtValue, ValuesJson, parse_json_points, save_json_points, smooth};
use errors::*;

#[cfg(test)]
mod test;
