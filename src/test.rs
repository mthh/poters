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
        let obs_points = parse_json_points("tests/disk1.json").unwrap();
        let configuration = Config::new(10.0, FuncNames::AmortizedDisk);
        let bbox = Bbox::new(-1.0, 6.0, -1.0, 6.0);
        smooth(80, 80, &bbox, &obs_points, configuration).unwrap();
    }

    #[test]
    fn test_parse_csv() {
        let obs_points = parse_csv_points("tests/data_1.in").unwrap();
        let configuration = Config::new(10.0, FuncNames::AmortizedDisk);
        let bbox = Bbox::new(-1.0, 6.0, -1.0, 6.0);
        smooth(80, 80, &bbox, &obs_points, configuration).unwrap();
    }

    #[test]
    fn test_parse_geojson() {
        let obs_points = parse_geojson_points("tests/input_ra.geojson", "value").unwrap();
        let configuration = Config::new(15.0, FuncNames::Gaussian);
        let bbox = Bbox::new(1.0, 4.0, 32.0, 35.0);
        smooth(80, 80, &bbox, &obs_points, configuration).unwrap();
    }

    #[test]
    fn test_compare_input_json_geojson() {
        let obs_points1 = parse_geojson_points("tests/input_ra.geojson", "value").unwrap();
        let obs_points2 = parse_json_points("tests/ra.json").unwrap();
        let bbox = Bbox::new(1.0, 4.0, 32.0, 35.0);
        let configuration = Config::new(12.5, FuncNames::Gaussian);
        let res1 = smooth(140, 70, &bbox, &obs_points1, configuration).unwrap();
        let configuration = Config::new(12.5, FuncNames::Gaussian);
        let res2 = smooth(140, 70, &bbox, &obs_points2, configuration).unwrap();
        assert_eq!(res1.len(), res2.len());
        assert_eq!(res1[0].len(), res2[0].len());
        let (len_i, len_j) = (res1.len(), res1[0].len());
        for i in 0..len_i {
            for j in 0..len_j {
                let (res_lat, res_lon, res_value) = res1[i][j].get_triplet();
                let (verif_lat, verif_lon, verif_value) = res2[i][j].get_triplet();
                assert_eq!(true, almost_equal(res_lat, verif_lat, 0.00001));
                assert_eq!(true, almost_equal(res_lon, verif_lon, 0.00001));
                assert_eq!(true, almost_equal(res_value, verif_value, 0.00001));
            }
        }
    }

    #[test]
    fn test_gaussian_against_hyantes_output() {
        let obs_points = parse_json_points("tests/ra.json").unwrap();
        let configuration = Config::new(15.0, FuncNames::Gaussian);
        let bbox = Bbox::new(1.0, 4.0, 32.0, 35.0);
        let res = smooth(160, 80, &bbox, &obs_points, configuration).unwrap();
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
        let obs_points = parse_json_points("tests/ra.json").unwrap();
        let configuration = Config::new(15.0, FuncNames::Exponential);
        let bbox = Bbox::new(1.0, 4.0, 32.0, 35.0);
        let res = smooth(160, 80, &bbox, &obs_points, configuration).unwrap();
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

    #[test]
    fn test_amortized_disk_against_hyantes_output() {
        let obs_points = parse_json_points("tests/ra.json").unwrap();
        let configuration = Config::new(15.0, FuncNames::AmortizedDisk);
        let bbox = Bbox::new(1.0, 4.0, 32.0, 35.0);
        let res = smooth(160, 80, &bbox, &obs_points, configuration).unwrap();
        let hyantes_verif_point = parse_json_points("tests/ra_amortized_output.json").unwrap();
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
