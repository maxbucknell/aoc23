use clap::{Parser, Subcommand};

use advent_of_code_2023::{Ex1A, Solution};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ex1a {

    }
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Ex1a {}) => Ok(Ex1A::new()),
        None => Err("No command specified")
    };

    match result {
        Ok(solution) => println!("{}", solution.solve()),
        Err(err) => println!("Error: {}", err)
    }
}
