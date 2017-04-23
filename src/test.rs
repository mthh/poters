#[cfg(test)]
mod test {
    use ::*;
    use std::f64;
    const PI:f64 = f64::consts::PI;

    pub fn almost_equal(a: f64, b: f64, epsilon: f64) -> bool {
    	let diff = (a - b).abs();
    	if a == b {
    		true
    	} else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
    		diff < (epsilon * f64::MIN_POSITIVE)
    	} else {
    		(diff / f64::min(a.abs() + b.abs(), f64::MAX)) < epsilon
    	}
    }
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

    #[test]
    fn test_gaussian_against_hyantes_output() {
        let mut obs_points = parse_json_points("tests/ra.json").unwrap();
        let configuration = Config::new(15.0, FuncNames::Gaussian);
        let bbox = Bbox::new(1.0, 4.0, 32.0, 35.0);
        let res = smooth(160, 80, bbox, &mut obs_points, configuration).unwrap();
        let hyantes_verif_point = parse_json_points("tests/ra_gaussian_output.json").unwrap();
        let mut flatten_res = Vec::new();
        for arr in res.iter(){
            for elem in arr.iter(){
                flatten_res.push(elem);
            }
        }
        assert_eq!(hyantes_verif_point.len(), flatten_res.len());
        for i in 0..flatten_res.len() {
            let (res_lat, res_lon, res_value) = flatten_res[i].get_triplet();
            let (verif_lat, verif_lon, verif_value) = hyantes_verif_point[i].get_triplet();
            assert_eq!(true, almost_equal(res_lat, verif_lat / PI * 180.0, 0.00001));
            assert_eq!(true, almost_equal(res_lon, verif_lon / PI * 180.0, 0.00001));
            assert_eq!(true, almost_equal(res_value, verif_value, 0.00001));

        }
    }

    #[test]
    fn test_exponential_against_hyantes_output() {
        let mut obs_points = parse_json_points("tests/ra.json").unwrap();
        let configuration = Config::new(15.0, FuncNames::Exponential);
        let bbox = Bbox::new(1.0, 4.0, 32.0, 35.0);
        let res = smooth(160, 80, bbox, &mut obs_points, configuration).unwrap();
        let hyantes_verif_point = parse_json_points("tests/ra_exponential_output.json").unwrap();
        let mut flatten_res = Vec::new();
        for arr in res.iter(){
            for elem in arr.iter(){
                flatten_res.push(elem);
            }
        }
        assert_eq!(hyantes_verif_point.len(), flatten_res.len());
        for i in 0..flatten_res.len() {
            let (res_lat, res_lon, res_value) = flatten_res[i].get_triplet();
            let (verif_lat, verif_lon, verif_value) = hyantes_verif_point[i].get_triplet();
            assert_eq!(true, almost_equal(res_lat, verif_lat / PI * 180.0, 0.00001));
            assert_eq!(true, almost_equal(res_lon, verif_lon / PI * 180.0, 0.00001));
            assert_eq!(true, almost_equal(res_value, verif_value, 0.00001));

        }
    }
}
