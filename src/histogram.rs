use crate::cli::Histogram;
use crate::utils;
use std::io::{self, BufRead};

// todo give option for left open or right open
/// Assumes data is sorted.
fn bin_data(data: &[f64], nbins: u8) -> Vec<u32> {
    let min = utils::get_min(&data);
    let max = utils::get_max(&data);
    let step = get_step(min, max, nbins);

    let mut bins = vec![0_u32; nbins as usize];

    // lowest of breaks guaranteed to be lower than lowest, and higher than highest data.
    let breaks = get_breaks_bins_specified(min, max, step, nbins);

    assert_eq!(bins.len(), breaks.len() - 1);

    for &datum in data.iter() {
        for i in 0..bins.len() {
            let left_break = breaks[i];
            let right_break = breaks[i + 1];

            // todo would be nice to make this some sort of tree structure
            if left_break <= datum && datum <= right_break {
                bins[i] += 1;

                break;
            }
        }
    }

    bins
}

fn get_bar_height(this_count: u32, max_count: u32, max_height: u8) -> u8 {
    assert!(this_count <= max_count);
    assert!(max_height > 0);

    (this_count as f64 / max_count as f64 * max_height as f64).ceil() as u8
}

fn get_breaks_bins_specified(min: f64, max: f64, step: f64, nbins: u8) -> Vec<f64> {
    assert!(min < max);
    assert!(step > 0.0);

    let mut v = vec![min];

    let mut current_break = min + step;

    for _i in 0..nbins {
        v.push(current_break);
        current_break += step;
    }

    v
}

fn get_step(min: f64, max: f64, num_bins: u8) -> f64 {
    assert!(min < max);
    assert!(num_bins > 0);

    if min < 0.0 && max <= 0.0 {
        (min.abs() - max.abs()) / num_bins as f64
    } else if min < 0.0 && max > 0.0 {
        (min.abs() + max) / num_bins as f64
    } else if min >= 0.0 && max > 0.0 {
        (max - min) / num_bins as f64
    } else {
        panic!("oops! shouldn't get here!");
    }
}

fn draw_bars(bar_heights: &[u8], bar_char: char, axis: &str) {
    println!();
    for &height in bar_heights {
        let bar: String = std::iter::repeat(bar_char).take(height as usize).collect();

        println!("{}{}", axis, bar);
    }
    println!();
}

pub fn histogram(opts: Histogram) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let numbers = lines
        .map(|line| {
            line.expect("couldn't get line")
                .parse::<f64>()
                .expect("couldn't parse line!")
        })
        .collect::<Vec<f64>>();

    eprintln!(
        "min: {:.2}, max: {:.2}",
        utils::get_min(&numbers),
        utils::get_max(&numbers)
    );

    let bins = bin_data(&numbers, opts.bins);
    let max_count = bins.iter().max().unwrap();

    let bar_heights = bins
        .iter()
        .map(|count| get_bar_height(*count, *max_count, opts.height))
        .collect::<Vec<u8>>();

    draw_bars(&bar_heights, opts.char, &opts.axis);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_get_bar_height {
        use super::*;

        #[test]
        fn get_bar_height_works() {
            assert_eq!(get_bar_height(0, 3, 100), 0);
            assert_eq!(get_bar_height(1, 3, 100), 34);
            assert_eq!(get_bar_height(2, 3, 100), 67);
            assert_eq!(get_bar_height(3, 3, 100), 100);

            // Even tiny ones get bumped up to 1
            assert_eq!(get_bar_height(1, 100, 1), 1);

            assert_eq!(get_bar_height(10, 10, 10), 10);
            assert_eq!(get_bar_height(10, 20, 10), 5);
            assert_eq!(get_bar_height(20, 20, 10), 10);
            assert_eq!(get_bar_height(0, 20, 10), 0);
        }

        #[test]
        #[should_panic]
        fn get_bar_height_panics_on_bad_counts() {
            get_bar_height(11, 10, 5);
        }

        #[test]
        #[should_panic]
        fn get_bar_height_panics_on_bad_height() {
            get_bar_height(5, 10, 0);
        }
    }

    mod test_get_step {
        use super::*;

        #[test]
        fn get_step_works() {
            assert_eq!(get_step(0.0, 10.0, 2), 5.0);
            assert_eq!(get_step(2.0, 10.0, 5), 1.6);
            assert_eq!(get_step(2.0, 10.0, 1), 8.0);
            assert_eq!(get_step(0.0, 10.0, 20), 0.5);
            assert_eq!(get_step(-10.0, 10.0, 1), 20.0);
            assert_eq!(get_step(-10.0, 5.0, 5), 3.0);
            assert_eq!(get_step(-7.0, 14.0, 5), 4.2);
        }

        #[test]
        #[should_panic]
        fn get_step_panics_with_bad_input() {
            get_step(0.0, 0.0, 3);
        }

        #[test]
        #[should_panic]
        fn get_step_panics_with_bad_input2() {
            get_step(0.0, 10.0, 0);
        }

        #[test]
        #[should_panic]
        fn get_step_panics_with_bad_input3() {
            get_step(0.0, -10.0, 5);
        }
    }
}
