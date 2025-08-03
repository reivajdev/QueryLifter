mod cli;
use clap::Parser; 
use cli::Cli;
use cli::runnable::Runnable; 

fn main() {
    let cli = Cli::parse();
    cli.command.run();
}