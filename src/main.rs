use clap::{Parser, Subcommand};

use advent_of_code_2023::Solution;
use advent_of_code_2023::ex1a::Ex1A;
use advent_of_code_2023::ex1b::Ex1B;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ex1a {},
    Ex1b {}
}

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        panic!("No command specified");
    }

    let solution: Box<dyn Solution> = match cli.command.unwrap() {
        Commands::Ex1a {} => Box::new(Ex1A::new()),
        Commands::Ex1b {} => Box::new(Ex1B::new())
    };

    println!("{}", solution.solve());
}
