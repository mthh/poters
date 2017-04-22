#[cfg(test)]
mod test {
    use ::*;
    #[test]
    fn test_amortized1() {
        let mut obs_points = parse_json_points("tests/disk1.json").unwrap();
        let configuration = Config::new(10.0, FuncNames::AmortizedDisk);
        let bbox = Bbox::new(-1.0, 6.0, -1.0, 6.0);
        let res = smooth(80, 80, bbox, &mut obs_points, configuration).unwrap();
    }
}
