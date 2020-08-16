use crate::range::Range;
use crate::utils;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub struct Points<T> {
    pub data: Vec<Point<T>>,
    pub xrange: Range<T>,
    pub yrange: Range<T>,
}

impl Points<f64> {
    pub fn get_points() -> Points<f64> {
        let stdin = io::stdin();
        let lines = stdin.lock().lines();

        let mut data = Vec::new();

        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;

        for line in lines {
            let line = line.expect("couldn't get line");

            let ary = line.split('\t').collect::<Vec<&str>>();
            assert_eq!(ary.len(), 2, "line doesn't have 2 tokens!");

            let x = ary[0].parse::<f64>().expect("couldn't parse x");
            let y = ary[1].parse::<f64>().expect("couldn't parse y");

            // Track x min max.
            if x < xmin {
                xmin = x;
            } else if x > xmax {
                xmax = x;
            }

            // Track y min max.
            if y < ymin {
                ymin = y;
            } else if y > ymax {
                ymax = y;
            }

            data.push(Point::<f64> { x, y });
        }

        Points {
            data,
            xrange: Range::<f64>::new(xmin, xmax),
            yrange: Range::<f64>::new(ymin, ymax),
        }
    }

    pub fn scale_points(
        self,
        new_xrange: &Range<usize>,
        new_yrange: &Range<usize>,
    ) -> Points<usize> {
        let new_data = self
            .data
            .iter()
            .map(|point| {
                let x = utils::scale(
                    point.x,
                    self.xrange.min,
                    self.xrange.max,
                    new_xrange.min as f64,
                    new_xrange.max as f64,
                );

                let y = utils::scale(
                    point.y,
                    self.yrange.min,
                    self.yrange.max,
                    new_yrange.min as f64,
                    new_yrange.max as f64,
                );

                let x = x.round();
                let y = y.round();

                assert!(0.0 <= x && x <= new_xrange.max as f64);
                assert!(0.0 <= y && y <= new_yrange.max as f64);

                Point::<usize> {
                    x: x as usize,
                    y: y as usize,
                }
            })
            .collect::<Vec<Point<usize>>>();

        Points {
            data: new_data,
            xrange: Range::<usize>::new(new_xrange.min as usize, new_xrange.max as usize),
            yrange: Range::<usize>::new(new_yrange.min as usize, new_yrange.max as usize),
        }
    }
}
