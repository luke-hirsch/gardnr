use crate::tech::Tech;
use std::{fs, io, path::PathBuf};

pub struct Component {
    pub name: String,
    pub tech: Tech,
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

    // Create component directories
    for component in &project.components {
        let component_dir = project_dir.join("components").join(&component.name);
        fs::create_dir_all(&component_dir)?;
        println!("Created component directory: {}", component_dir.display());
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
