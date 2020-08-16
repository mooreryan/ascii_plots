use asciiplot::cli::Subcommands;
use structopt::StructOpt;

fn main() {
    let args = Subcommands::from_args();

    asciiplot::run(args);
}
