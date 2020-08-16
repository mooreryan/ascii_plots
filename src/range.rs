use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
    pub range: T,
}

impl<T: Display> Display for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "min: {:.2}, max: {:.2}, range: {:.2}",
            self.min, self.max, self.range
        )
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    mod test_new_range_f64 {
        use super::*;

        #[test]
        fn with_all_positive_numbers_works() {
            let range = Range::<f64>::new(0.0, 10.0);
            assert_eq!(range.range, 10.0);

            let range = Range::<f64>::new(1.0, 10.0);
            assert_eq!(range.range, 9.0);
        }

        #[test]
        fn with_all_negative_numbers_works() {
            let range = Range::<f64>::new(-10.0, 0.0);
            assert_eq!(range.range, 10.0);

            let range = Range::<f64>::new(-10.0, -1.0);
            assert_eq!(range.range, 9.0);
        }

        #[test]
        fn with_mixed_numbers_works() {
            let range = Range::<f64>::new(-10.0, 10.0);
            assert_eq!(range.range, 20.0);
        }
    }
}
