pub fn median(data: &mut Vec<f64>) -> f64 {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if data.len() % 2 == 0 {
        let middle_cursor = data.len() / 2;
        return ((data[middle_cursor - 1] + data[middle_cursor]) as f64) / 2.0;
    } else {
        return data[data.len() / 2] as f64;
    }
}

pub fn max(data: &mut Vec<f64>) -> f64 {
    return *data.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
}

pub fn min(data: &mut Vec<f64>) -> f64 {
    return *data.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max() {
        let mut data: Vec<f64> = Vec::new();
        data.push(1.0);
        data.push(10.0);
        data.push(5.0);
        data.push(30.0);
        assert_eq!(30.0, max(&mut data));
    }

    #[test]
    fn test_min() {
        let mut data: Vec<f64> = Vec::new();
        data.push(1.0);
        data.push(10.0);
        data.push(5.0);
        data.push(30.0);
        assert_eq!(1.0, min(&mut data));
    }

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
