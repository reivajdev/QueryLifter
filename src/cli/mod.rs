pub mod commands;
pub mod runnable;

use clap::{Parser, Subcommand};
use commands::{Init, Run};

#[derive(Parser)]
#[command(name = "querylifter")]
#[command(about = "CLI para gestión de scripts SQL por entorno")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init {
        #[arg(short, long)]
        name: String,
    },
    Run {
        #[arg(short, long)]
        env: String,
        #[arg(short, long)]
        path: String,
    },
}

impl Runnable for Command {
    fn run(&self) {
        match self {
            Command::Init { name } => Init {
                name: name.clone(),
            }
            .run(),
            Command::Run { env, path } => Run {
                env: env.clone(),
                path: path.clone(),
            }
            .run(),
        }
    }
}

pub use runnable::Runnable;
