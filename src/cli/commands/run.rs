use crate::cli::runnable::Runnable;

pub struct Run {
    pub env: String,
    pub path: String,
}

impl Runnable for Run {
    fn run(&self) {
        println!("Ejecutando scripts en entorno '{}' desde '{}'", self.env, self.path);
    }
}