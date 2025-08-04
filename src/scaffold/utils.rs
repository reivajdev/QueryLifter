use crate::scaffold::layer::Layer;

fn normalize_key(s: &str) -> String {
    s.to_lowercase()
        .replace('.', "_")
        .replace('-', "_")
        .replace(' ', "_")
        .replace('/', "_")
}

pub fn layer_to_properties(layers: &[String]) -> Vec<Layer> {
    layers.iter()
            .map(|name| Layer {
                name: name.clone(),
                key: normalize_key(name),
            })
            .collect()
}