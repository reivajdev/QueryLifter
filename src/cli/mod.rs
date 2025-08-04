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
        /// Capas a crear (por defecto: BronzeData,SilverData,GoldData)
        #[arg(long, value_delimiter = ',', default_value = "BronzeData,SilverData,GoldData")]
        layers: Vec<String>,

        /// Entornos a configurar (por defecto: dev,pre,pro)
        #[arg(long, value_delimiter = ',', default_value = "dev,pre,pro")]
        envs: Vec<String>,
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
            Command::Init { name, layers, envs } => Init {
                name: name.to_owned(),
                layers: layers.clone(),
                envs: envs.clone(),
            }.run(),
            Command::Run { env, path } => Run {
                env: env.clone(),
                path: path.clone(),
            }
            .run(),
        }
    }
}

pub use runnable::Runnable;
