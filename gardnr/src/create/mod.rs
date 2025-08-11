use crate::tech::node::scaffold_js_project;
use crate::tech::python::scaffold_python_project;
use std::{fs, io, path::PathBuf, process::Command};

pub struct Component {
    pub name: String,
    pub tech: String,
}
pub struct Project {
    pub name: String,
    pub path: String,
    pub components: Vec<Component>,
}

pub fn create_project(project: Project) -> io::Result<()> {
    // Normalize path
    let mut project_path = PathBuf::from(project.path.trim());
    if project.path.trim().is_empty() {
        project_path = PathBuf::from(".".to_string());
    }

    // Create project directory
    let project_dir = project_path.join(&project.name);

    // Check if directory exists
    if project_dir.exists() {
        println!(
            "Project directory already exists: {}",
            project_dir.display()
        );
        return Ok(());
    }

    // Create directory
    fs::create_dir(&project_dir)?;
    println!("Created project directory: {}", project_dir.display());

    // Create and scaffold component directories
    for component in &project.components {
        let tech_lower = component.tech.to_lowercase();
        // For Django and Vite-based projects, let the scaffolding tools create the directory
        if !matches!(
            tech_lower.as_str(),
            "django" | "react" | "vue" | "svelte" | "nextjs" | "next" | "nuxt"
        ) {
            let component_dir = project_dir.join(&component.name);
            fs::create_dir_all(&component_dir)?;
            println!("Created component directory: {}", component_dir.display());
        }

        // Scaffold technology-specific structure
        if let Err(e) = scaffold_component(&project_dir, &component.name, &component.tech) {
            eprintln!("Warning: Failed to scaffold {}: {}", component.tech, e);
        }
    }

    Ok(())
}

pub fn questionnaire() -> io::Result<()> {
    println!("Enter project name (cannot be empty):");
    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read project name");

    let project_name = project_name.trim().to_string();
    if project_name.is_empty() {
        eprintln!("Project name cannot be empty!");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Project name cannot be empty",
        ));
    }

    println!("Enter project path (empty for current directory):");
    let mut project_path = String::new();
    io::stdin()
        .read_line(&mut project_path)
        .expect("Failed to read project path");

    let project_path = project_path.trim().to_string();

    let mut components = Vec::new();

    loop {
        println!("Enter component name (or leave empty to finish):");
        let mut component_name = String::new();
        io::stdin()
            .read_line(&mut component_name)
            .expect("Failed to read component name");

        let component_name = component_name.trim().to_string();
        if component_name.is_empty() {
            break;
        }

        println!(
            "Enter technology for {} (e.g., Rust, Python):",
            component_name
        );
        let mut component_tech = String::new();
        io::stdin()
            .read_line(&mut component_tech)
            .expect("Failed to read component technology");

        let component_tech = component_tech.trim().to_string();
        if component_tech.is_empty() {
            eprintln!("Technology cannot be empty!");
            continue;
        }

        components.push(Component {
            name: component_name,
            tech: component_tech,
        });
    }

    let new_project = Project {
        name: project_name,
        path: project_path,
        components,
    };

    create_project(new_project)
}

fn scaffold_component(base_dir: &PathBuf, component_name: &str, tech: &str) -> io::Result<()> {
    let component_dir = base_dir.join(component_name);
    let tech_lower = tech.to_lowercase();

    match tech_lower.as_str() {
        // Python-based technologies
        "django" | "flask" | "fastapi" | "pyramid" | "python" => {
            scaffold_python_project(base_dir, component_name, tech)
        }
        // JavaScript/Node.js-based technologies
        "react" | "vue" | "svelte" | "node" | "nodejs" | "express" | "nextjs" | "next" | "nuxt" => {
            scaffold_js_project(base_dir, component_name, tech)
        }
        "rust" => scaffold_rust(&component_dir, component_name),
        _ => {
            // Check if it might be a Python library
            if is_likely_python_package(tech) {
                scaffold_python_project(base_dir, component_name, tech)
            } else if is_likely_node_package(tech) {
                scaffold_js_project(base_dir, component_name, tech)
            } else {
                println!(
                    "→ No specific scaffolding for '{}', created basic directory",
                    tech
                );
                Ok(())
            }
        }
    }
}

fn is_likely_python_package(tech: &str) -> bool {
    // List of common Python packages/frameworks
    let python_packages = [
        "numpy",
        "pandas",
        "matplotlib",
        "scipy",
        "scikit-learn",
        "sklearn",
        "tensorflow",
        "torch",
        "pytorch",
        "keras",
        "opencv",
        "cv2",
        "requests",
        "aiohttp",
        "celery",
        "redis",
        "sqlalchemy",
        "jupyter",
        "notebook",
        "streamlit",
        "gradio",
    ];

    python_packages
        .iter()
        .any(|&pkg| tech.to_lowercase().contains(pkg))
}

fn is_likely_node_package(tech: &str) -> bool {
    // List of common Node.js packages/frameworks
    let node_packages = [
        "express",
        "koa",
        "fastify",
        "hapi",
        "socket.io",
        "ws",
        "axios",
        "lodash",
        "moment",
        "chalk",
        "commander",
        "inquirer",
        "jest",
        "mocha",
        "webpack",
        "rollup",
        "eslint",
        "prettier",
        "typescript",
        "babel",
        "angular",
        "ember",
    ];

    node_packages
        .iter()
        .any(|&pkg| tech.to_lowercase().contains(pkg))
}

fn scaffold_rust(component_dir: &PathBuf, component_name: &str) -> io::Result<()> {
    println!("→ Scaffolding Rust project");

    let output = Command::new("cargo").arg("--version").output();

    if output.is_err() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "cargo not found. Install Rust first",
        ));
    }

    let parent_dir = component_dir.parent().unwrap();
    let status = Command::new("cargo")
        .arg("new")
        .arg(component_name)
        .current_dir(parent_dir)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "cargo new failed"));
    }

    println!("→ Rust project scaffolded successfully");
    Ok(())
}
