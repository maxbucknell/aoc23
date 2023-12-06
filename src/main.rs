use clap::{Parser, Subcommand};

use advent_of_code_2023::Solution;
use advent_of_code_2023::ex1a::Ex1A;
use advent_of_code_2023::ex1b::Ex1B;
use advent_of_code_2023::ex2a::Ex2A;
use advent_of_code_2023::ex2b::Ex2B;
use advent_of_code_2023::ex3a::Ex3A;
use advent_of_code_2023::ex3b::Ex3B;
use advent_of_code_2023::ex4a::Ex4A;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ex1a {},
    Ex1b {},
    Ex2a {},
    Ex2b {},
    Ex3a {},
    Ex3b {},
    Ex4a {}
}

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        panic!("No command specified");
    }

    let solution: Box<dyn Solution> = match cli.command.unwrap() {
        Commands::Ex1a {} => Box::new(Ex1A::new()),
        Commands::Ex1b {} => Box::new(Ex1B::new()),
        Commands::Ex2a {} => Box::new(Ex2A::new()),
        Commands::Ex2b {} => Box::new(Ex2B::new()),
        Commands::Ex3a {} => Box::new(Ex3A::new()),
        Commands::Ex3b {} => Box::new(Ex3B::new()),
        Commands::Ex4a {} => Box::new(Ex4A::new()),
    };

    println!("{}", solution.solve());
}
