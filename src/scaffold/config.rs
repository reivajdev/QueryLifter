use std::fs::{self, write};
use chrono::Local;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};
use crate::scaffold::layer::Layer;
use std::path::MAIN_SEPARATOR;

/// Crea la carpeta raíz del proyecto
const DEPLOYRC_TEMPLATE: &str = include_str!("../../templates/.deployrc.tpl");

pub fn create_project_directory(name: &str) -> std::io::Result<PathBuf> {
    let input_path = PathBuf::from(name);
    let project_path = if input_path.is_absolute() {
        input_path
    } else {
        std::env::current_dir()?.join(input_path)
    };

    if project_path.exists() {
        println!("⚠️  La carpeta ya existe: {}", project_path.display());
    } else {
        fs::create_dir_all(&project_path)?;
    }

    Ok(project_path)
}

/// Crea carpetas fijas como logs y dbscripts/
pub fn create_subfolders(base_path: &Path) -> std::io::Result<()> {
    let subfolders: Vec<&'static str> = vec!["logs", "dbscripts"];
    for name in subfolders {
        let sub_path = base_path.join(name);
        if !sub_path.exists() {
            fs::create_dir_all(&sub_path)?;
            println!("✅ Carpeta creada: {}", sub_path.display());
        }
    }
    Ok(())
}

/// Crea carpetas dbscripts/<name>/<env>
pub fn create_layer_folders(
    base_path: &Path,
    base_folder: &str,
    layers: &[Layer],
    envs: &[String],
) -> std::io::Result<()> {
    for layer in layers {
        for env in envs {
            let path = base_path.join(base_folder).join(&layer.name).join(env);
            if !path.exists() {
                fs::create_dir_all(&path)?;
                println!("✅ Carpeta creada: {}", path.display());
            }
        }
    }
    Ok(())
}

pub fn create_deployrc(
    path: &Path,
    project_name: &str,
    layers: &[Layer],
    envs: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("name", project_name.rsplit(MAIN_SEPARATOR).last().unwrap());
    context.insert("created_at", &Local::now().format("%Y-%m-%d").to_string());
    context.insert("layers", layers);

    let mut env_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    for env in envs {
        let mut dbs = HashMap::new();
        for layer in layers {
            let key = format!("{}_db_url", layer.key);
            let val = format!(
                "postgresql://user:pass@localhost:5432/{}_{}",
                env,
                layer.key
            );
            dbs.insert(key, val);
        }
        env_map.insert(env.clone(), dbs);
    }

    context.insert("environments", &env_map);

    // Si quieres usar include_str!, cambia esto por Tera::one_off(...)
    let rendered = Tera::one_off(DEPLOYRC_TEMPLATE, &context, false)?;
    let config_path = path.join(".deployrc");
    write(&config_path, rendered)?;
    println!("✅ Archivo .deployrc creado en: {}", config_path.display());

    Ok(())
}