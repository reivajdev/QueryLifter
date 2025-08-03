use crate::cli::runnable::Runnable;

pub struct Init {
    pub name: String,
}

impl Runnable for Init {
    fn run(&self) {
        println!("Inicializando proyecto: {}", self.name);
    }
}