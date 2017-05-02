extern crate clap;
extern crate poters;
#[macro_use] extern crate scan_rules;

use poters::*;
use clap::{Arg, App};

fn main(){
    let matches = App::new("poters").version("0.1.0")
       .about("Compute neighbourhood population potential")
       .arg(Arg::with_name("input")
            .short("i").long("input")
            .required(true).takes_value(true)
            .value_name("FILE")
            .help("Input file to use (.csv or .json)."))
        .arg(Arg::with_name("function")
            .short("f").long("function")
            .required(true).takes_value(true)
            .value_name("FUNCTION")
            .help("Name of the smoothing function, chosen among disk, amortized_disk, gaussian, pareto, exponential."))
        .arg(Arg::with_name("range")
             .short("r").long("range")
             .required(true).takes_value(true)
             .value_name("RANGE")
             .help("Smoothing range in kilometers, used as parameter of the interaction function."))
        .arg(Arg::with_name("scale")
             .short("s").long("scale")
             .required(true).takes_value(true)
             .value_name("SCALE")
             .help("Resolution of the output in kilometers as ResoLat-ResoLon."))
        .arg(Arg::with_name("window")
             .short("w").long("window")
             .required(true).takes_value(true).require_equals(true)
             .value_name("SCALE")
             .help("Coordinate of the visualisation window, given in degrees as minimum latitude,minimum longitude,maximum latitude,maximum longitude."))
        .arg(Arg::with_name("output")
             .short("o").long("output")
             .required(true).takes_value(true)
             .value_name("FILE")
             .help("Path for output file (json format)."))
        .arg(Arg::with_name("field")
            .short("c").long("field_name")
            .takes_value(true)
            .value_name("FIELD")
            .help("(Required for GeoJSON input) Field name containing the stock values to use."))
        .get_matches();
    let file_path = matches.value_of("input").unwrap();
    let obs_points = if file_path.contains("geojson") || file_path.contains("GEOJSON") {
        let field_name = if matches.is_present("field") { matches.value_of("field") } else { None };
        if field_name.is_none(){
            panic!("Error: Field name is required for GeoJSON input (arg. --field=name).");
        }
        parse_geojson_points(file_path, field_name.unwrap()).unwrap()
    } else if file_path.contains("json") || file_path.contains("JSON") {
        parse_json_points(file_path).unwrap()
    } else {
        parse_csv_points(file_path).unwrap()
    };
    let func_name = match matches.value_of("function") {
        Some("disk") => FuncNames::Disk,
        Some("amortized_disk") => FuncNames::AmortizedDisk,
        Some("gaussian") => FuncNames::Gaussian,
        Some("exponential") => FuncNames::Exponential,
        Some("pareto") => FuncNames::Pareto,
        Some(&_) | None => panic!("Invalid smoothing function name. Expected one of: disk, amortized_disk, gaussian, pareto, exponential. ")
    };
    let range: f64 = matches.value_of("range").unwrap().parse::<f64>().unwrap();
    let_scan!(matches.value_of("scale").unwrap(); (
        let reso_lat: u32, "-", let reso_lon: u32));
    let_scan!(matches.value_of("window").unwrap(); (
        let min_lat: f64, ",", let max_lat: f64, ",", let min_lon: f64, ",", let max_lon: f64));
    let configuration = Config::new(range, func_name);
    let bbox = Bbox::new(min_lat, max_lat, min_lon, max_lon);
    let res = smooth(reso_lat as u32, reso_lon as u32, &bbox, &obs_points, configuration).unwrap();
    let output_path = matches.value_of("output").unwrap();
    if output_path.contains("geojson") {
        save_geojson_points(output_path, res).unwrap();
    } else {
        save_json_points(output_path, res).unwrap();
    }
}
