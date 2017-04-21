#[cfg(test)]
mod test {
    use ::*;
    #[test]
    fn test_amortized1() {
        let mut obs_points = parse_json_points("tests/disk1.json");
        let configuration = Config {fparam: 10.0, smoothing_fun_t: FuncNames::AmortizedDisk};
        let bbox = Bbox {min_lat: -1.0, max_lat: 6.0, min_lon: -1.0, max_lon: 6.0};
        let res = smooth(80, 80, bbox, &mut obs_points, configuration);
    }
}
