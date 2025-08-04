use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Layer {
    pub name: String,
    pub key: String,
}