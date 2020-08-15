pub mod cli;
mod histogram;
mod utils;

use crate::cli::Subcommands;

pub fn run(args: Subcommands) {
    match args {
        Subcommands::Histogram(opts) => {
            histogram::histogram(opts);
        }
    }
}
