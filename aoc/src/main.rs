use aoc::show_solutions;
use aoc::Options;
use clap::Parser;

/// Run Advent of Code (AoC) solutions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The AoC year to run solutions from
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(2015..=2015))]
    year: u16,

    /// The AoC date to run
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    date: Option<u8>,

    /// The solution part to run
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,
}

fn main() {
    // let args = Args::parse();

    show_solutions(Options {
        year: 2015,
        day: 1,
        part: None,
    });
}
