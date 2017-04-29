extern crate poters;
use poters::*;

fn main(){
    let obs_points = parse_json_points("tests/disk1.json").unwrap();
    let configuration = Config::new(10.0, FuncNames::AmortizedDisk);
    let bbox = Bbox::new(-1.0, 6.0, -1.0, 6.0);
    let res = smooth(80, 80, &bbox, &obs_points, configuration).unwrap();
    save_json_points("/tmp/result_disk1.json", res).unwrap();
}
