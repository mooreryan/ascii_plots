use crate::cli::Scatterplot;
use crate::point::Points;
use crate::range::Range;

pub fn scatterplot(opts: Scatterplot) {
    if opts.aspect_ratio != 0.0 {
        panic!("todo implement fixed aspect ratio");
    }

    let points = Points::get_points();

    eprintln!("x range -- {}\ny range -- {}", points.xrange, points.yrange);

    let viewer_xrange = Range::<usize>::new(0_usize, opts.width - 1);
    let viewer_yrange = Range::<usize>::new(0_usize, opts.height - 1);

    let scaled_points = points.scale_points(&viewer_xrange, &viewer_yrange);

    let mut plot = vec![vec![' '; opts.width]; opts.height];
    for point in scaled_points.data {
        let row_idx = viewer_yrange.max - point.y;
        let col_idx = point.x;

        plot[row_idx][col_idx] = opts.char;
    }

    let horizontal_border = std::iter::repeat('-').take(opts.width).collect::<String>();

    println!("\n|{}|", &horizontal_border);
    for row in plot {
        let row = row.into_iter().collect::<String>();
        println!("|{}|", row);
    }
    println!("|{}|\n", &horizontal_border);
}
