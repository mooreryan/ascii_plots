use structopt::StructOpt;

/// Super smashing program here!
#[derive(Debug, StructOpt)]
pub enum Subcommands {
    Histogram(Histogram),
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

// todo allow float steps...better for small numbers
