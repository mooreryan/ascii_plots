use structopt::StructOpt;

/// Draw neat plots in your terminal!
#[derive(Debug, StructOpt)]
pub enum Subcommands {
    Histogram(Histogram),
    Scatterplot(Scatterplot),
}

/// Draw histograms
///
/// Input should be one column of numbers.
#[derive(Debug, StructOpt)]
pub struct Histogram {
    // better default would be to calculate a good one
    /// Step for the data range.
    #[structopt(long, default_value = "30")]
    pub bins: u8,

    /// How long should the bars be?
    #[structopt(long, default_value = "80")]
    pub height: u8,

    /// What char to use for the bars?
    #[structopt(long, default_value = "*")]
    pub char: char,

    /// What do you want for the axis?
    #[structopt(long, default_value = "| ")]
    pub axis: String,
}

/// Draw scatterplots
///
/// Input should be two columns of numbers: 1st is x-values, 2nd is y-values.
#[derive(Debug, StructOpt)]
pub struct Scatterplot {
    /// What char to use for the points?
    #[structopt(long, default_value = "*")]
    pub char: char,

    /// Chart width
    #[structopt(long, default_value = "80")]
    pub width: usize,

    /// Chart height
    #[structopt(long, default_value = "20")]
    pub height: usize,
}
