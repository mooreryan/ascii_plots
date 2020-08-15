#![allow(dead_code)]

pub fn get_max(v: &[f64]) -> f64 {
    v.iter().fold(f64::NAN, |max, x| f64::max(max, *x))
}

pub fn get_min(v: &[f64]) -> f64 {
    v.iter().fold(f64::NAN, |min, x| f64::min(min, *x))
}

/// See https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=b2bc6c807fd63b399eaac06391858496.
pub fn round(x: f64, precision: f64) -> f64 {
    (x / precision).round() * precision
}

pub fn range_start(min: f64, step: f64) -> f64 {
    (min / step).floor() * step
}

pub fn range_stop(max: f64, step: f64) -> f64 {
    (max / step).ceil() * step
}

/// Assumes that step % (max - min).abs() == 0
/// this should really be for the scatter plots
pub fn get_x_axis_ticks(min: f64, max: f64, step: f64) -> Vec<f64> {
    assert!(min < max);
    assert!(step > 0.0);

    let min = range_start(min, step);
    let max = range_stop(max, step);

    // Now min, and max should be divisible by step.
    //
    // Come up with some good tolerance.
    // See https://doc.rust-lang.org/std/primitive.f64.html#method.rem_euclid.
    assert!((max - min).abs().rem_euclid(step) < 0.0001);

    let mut v = vec![min];

    let mut thing = min + step;

    while thing <= max {
        v.push(thing);
        thing += step;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_get_breaks {
        use super::*;

        #[test]
        fn breaks_works() {
            assert_eq!(get_x_axis_ticks(0.0, 10.0, 5.0), [0.0, 5.0, 10.0]);
            assert_eq!(get_x_axis_ticks(0.0, 5.0, 5.0), [0.0, 5.0]);
            assert_eq!(get_x_axis_ticks(-5.0, 5.0, 5.0), [-5.0, 0.0, 5.0]);

            assert_eq!(get_x_axis_ticks(0.3, 9.8, 5.0), [0.0, 5.0, 10.0]);
            assert_eq!(get_x_axis_ticks(0.3, 4.4, 5.0), [0.0, 5.0]);
            assert_eq!(get_x_axis_ticks(-0.1, 4.9999, 5.0), [-5.0, 0.0, 5.0]);
        }

        #[test]
        #[should_panic]
        fn breaks_panics_when_min_lt_max() {
            get_x_axis_ticks(10.0, 0.0, 5.0);
        }

        #[test]
        #[should_panic]
        fn breaks_panics_when_min_eq_max() {
            get_x_axis_ticks(10.0, 10.0, 5.0);
        }
    }
}
