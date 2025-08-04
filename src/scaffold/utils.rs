pub fn normalize_key(s: &str) -> String {
    s.to_lowercase()
        .replace('.', "_")
        .replace('-', "_")
        .replace(' ', "_")
        .replace('/', "_")
}