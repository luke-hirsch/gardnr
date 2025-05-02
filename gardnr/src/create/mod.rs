use std::{fs::create_dir, io, path::PathBuf};

pub fn create_project(name: String, path: String) {
    let project_path = if path.trim().is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(path.trim())
    };

    let project_path = project_path.join(&name);
    create_dir(&project_path).expect("Failed to create project directory");
    println!("Creating project {} at {}!", name, path);
}

pub fn questionaire() {
    println!("How do you want to call your new project?");
    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read project name");
    project_name = project_name.trim().to_string();

    println!("Specific location? (empty for cwd)");
    let mut project_path = String::new();
    io::stdin()
        .read_line(&mut project_path)
        .expect("Failed to read project name");
    let project_path = if project_path.trim().is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(project_path.trim())
    };

    let project_path = project_path.join(&project_name);
    create_dir(&project_path).expect("Failed to create project directory");
    println!(
        "Creating project {} at {}!",
        project_name,
        project_path.to_string_lossy().to_string()
    );
}
