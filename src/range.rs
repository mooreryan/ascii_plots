#[derive(Debug)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
    pub range: T,
}

impl Range<f64> {
    pub fn new(min: f64, max: f64) -> Self {
        Range {
            min,
            max,
            range: Self::get_range(min, max),
        }
    }

    pub fn get_range(min: f64, max: f64) -> f64 {
        assert!(min <= max);

        if min < 0.0 && max <= 0.0 {
            min.abs() - max.abs()
        } else if min < 0.0 && max > 0.0 {
            min.abs() + max
        } else if min >= 0.0 && max > 0.0 {
            max - min
        } else {
            panic!("oops! impossible to get here!");
        }
    }
}

impl Range<usize> {
    pub fn new(min: usize, max: usize) -> Self {
        Range {
            min,
            max,
            range: Self::get_range(min, max),
        }
    }

    pub fn get_range(min: usize, max: usize) -> usize {
        assert!(min <= max);

        max - min
    }
}
