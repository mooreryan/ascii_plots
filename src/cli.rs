use structopt::StructOpt;

/// Super smashing program here!
#[derive(Debug, StructOpt)]
pub enum Subcommands {
    Histogram(Histogram),
    Scatterplot(Scatterplot),
}

/// Boop beep bop
#[derive(Debug, StructOpt)]
pub struct Histogram {
    // better default would be to calculate a good one
    /// Step for the data range.
    #[structopt(long, default_value = "30")]
    pub bins: u8,

    /// How long should the bars be?
    #[structopt(long, default_value = "70")]
    pub height: u8,

    /// What char to use for the bars?
    #[structopt(long, default_value = "◾")]
    pub char: char,

    /// What do you want for the axis?
    #[structopt(long, default_value = "| ")]
    pub axis: String,
}

/// Silly thing
#[derive(Debug, StructOpt)]
pub struct Scatterplot {
    /// What char to use for the points?
    #[structopt(long, default_value = "*")]
    pub char: char,

    /// What aspect ratio?
    #[structopt(long, default_value = "0")]
    pub aspect_ratio: f64,

    #[structopt(long, default_value = "80")]
    pub width: usize,

    #[structopt(long, default_value = "20")]
    pub height: usize,
}

// todo allow float steps...better for small numbers
