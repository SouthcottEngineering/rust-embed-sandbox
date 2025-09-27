pub mod hw;

pub use crate::hw::*;

/// Normalize an ID string for safe usage
pub fn normalize_id(id: &str) -> String {
    id.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>()
        .to_lowercase()
}
