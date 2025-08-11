pub mod db;
pub mod node;
pub mod python;
pub mod rust;

pub struct Tech {
    pub name: String,
    pub aliases: Vec<String>,
}

impl Tech {
    pub fn from_name(name: &str) -> Option<Tech> {
        match name.to_lowercase().as_str() {
            "rust" => Some(Tech {
                name: "Rust".to_string(),
                aliases: vec!["rust".to_string(), "rs".to_string()],
            }),
            "python" => Some(Tech {
                name: "Python".to_string(),
                aliases: vec![
                    "python".to_string(),
                    "py".to_string(),
                    "python3".to_string(),
                ],
            }),
            "node" | "nodejs" => Some(Tech {
                name: "Node.js".to_string(),
                aliases: vec!["node".to_string(), "nodejs".to_string(), "js".to_string()],
            }),
            "react" => Some(Tech {
                name: "React".to_string(),
                aliases: vec!["react".to_string(), "reactjs".to_string()],
            }),
            "vue" => Some(Tech {
                name: "Vue.js".to_string(),
                aliases: vec!["vue".to_string(), "vuejs".to_string()],
            }),
            "django" => Some(Tech {
                name: "Django".to_string(),
                aliases: vec!["django".to_string()],
            }),
            "flask" => Some(Tech {
                name: "Flask".to_string(),
                aliases: vec!["flask".to_string()],
            }),
            "fastapi" => Some(Tech {
                name: "FastAPI".to_string(),
                aliases: vec!["fastapi".to_string(), "fast-api".to_string()],
            }),
            _ => None,
        }
    }
}
