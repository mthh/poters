extern crate poters;
use poters::*;

fn main(){
    let mut obs_points = parse_json_points("tests/ra.json").unwrap();
    let configuration = Config::new(15.0, FuncNames::Pareto);
    let bbox = Bbox::new(1.0, 4.0, 32.0, 35.0);
    let res = smooth(160, 80, bbox, &mut obs_points, configuration).unwrap();
    save_json_points("/tmp/result_pareto.json", res).unwrap();
}
