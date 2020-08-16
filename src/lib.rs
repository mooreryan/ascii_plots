pub mod cli;
mod histogram;
mod point;
mod range;
mod scatterplot;
mod utils;

use crate::cli::Subcommands;

pub fn run(args: Subcommands) {
    match args {
        Subcommands::Histogram(opts) => {
            histogram::histogram(opts);
        }
        Subcommands::Scatterplot(opts) => {
            scatterplot::scatterplot(opts);
        }
    }
}
