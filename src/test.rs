#[cfg(test)]
mod test {
    use ::*;
    #[test]
    fn test_parse_json() {
        let mut obs_points = parse_json_points("tests/disk1.json").unwrap();
        let configuration = Config::new(10.0, FuncNames::AmortizedDisk);
        let bbox = Bbox::new(-1.0, 6.0, -1.0, 6.0);
        let res = smooth(80, 80, bbox, &mut obs_points, configuration).unwrap();
    }

    #[test]
    fn test_parse_csv() {
        let mut obs_points = parse_csv_points("tests/data_1.in").unwrap();
        let configuration = Config::new(10.0, FuncNames::AmortizedDisk);
        let bbox = Bbox::new(-1.0, 6.0, -1.0, 6.0);
        let res = smooth(80, 80, bbox, &mut obs_points, configuration).unwrap();
    }
}
