pub fn median(data: &mut Vec<f64>) -> f64 {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if data.len() % 2 == 0 {
        let middle_cursor = data.len() / 2;
        return ((data[middle_cursor - 1] + data[middle_cursor]) as f64) / 2.0;
    } else {
        return data[data.len() / 2] as f64;
    }
}

mod test {
    use super::median;

    #[test]
    fn test_median_even() {
        let mut data: Vec<f64> = Vec::new();
        data.push(1.0);
        data.push(10.0);
        data.push(5.0);
        data.push(30.0);
        assert_eq!(7.5, median(&mut data));
    }

    #[test]
    fn test_median_odd() {
        let mut data: Vec<f64> = Vec::new();
        data.push(1.0);
        data.push(10.0);
        data.push(5.0);
        data.push(30.0);
        data.push(20.0);
        assert_eq!(10.0, median(&mut data));
    }
}
