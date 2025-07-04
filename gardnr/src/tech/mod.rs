use super::utils::TechExecutables;

pub mod db;
pub mod node;
pub mod python;
pub mod rust;

pub struct Tech {
    name: &str,
    alias: Vec<String>,
}
