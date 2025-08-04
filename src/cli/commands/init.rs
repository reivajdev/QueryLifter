use crate::cli::runnable::Runnable;
use clap::Args;
use crate::scaffold::config::{create_project_directory, create_subfolders, create_layer_folders,  create_deployrc };
use crate::scaffold::layer::{Layer};
use crate::scaffold::utils::{layer_to_properties};

#[derive(Args, Debug)]
pub struct Init {
    #[arg(long)]
    pub name: String,

    #[arg(long, value_delimiter = ',', default_value = "BronzeData,SilverData,GoldData")]
    pub layers: Vec<String>,

    #[arg(long, value_delimiter = ',', default_value = "dev,pre,pro")]
    pub envs: Vec<String>,
}

impl Runnable for Init {
    fn run(&self) {
        println!("Inicializando proyecto: {}", self.name);

        let layers: Vec<Layer> = layer_to_properties(&self.layers);

        match create_project_directory(&self.name) {
            Ok(path) => {
                println!("✅ Proyecto inicializado en: {}", path.display());

                if let Err(e) = create_subfolders(&path) {
                    eprintln!("❌ Error al crear subcarpetas: {}", e);
                }

                if let Err(e) = create_layer_folders(&path, "dbscripts", &layers, &self.envs) {
                    eprintln!("❌ Error al crear capas + entornos: {}", e);
                }

                if let Err(e) = create_deployrc(&path, &self.name, &layers, &self.envs) {
                    eprintln!("❌ Error al crear .deployrc: {}", e);
                }
            }
            Err(e) => eprintln!("❌ Error al crear la carpeta del proyecto: {}", e),
        }
    }
}