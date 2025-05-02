use std::{
    env, fs,
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");
    buf.trim().to_string()
}

fn is_installed(cmds: &[&str]) -> Option<String> {
    for &cmd in cmds {
        if Command::new("which")
            .arg(cmd)
            .stdout(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return Some(cmd.to_string());
        }
    }
    None
}

fn prompt_clone(proj_dir: &Path) -> bool {
    let ans = prompt_input("Clone remote git repo into this folder? (y/N): ");
    if ans.to_lowercase() == "y" {
        let repo = prompt_input("Repository URL: ");
        println!("→ git clone {} {:?}", repo, proj_dir);
        let _ = Command::new("git")
            .arg("clone")
            .arg(&repo)
            .arg(proj_dir.to_str().unwrap())
            .status();
        return true;
    }
    false
}

fn scaffold_django(base: &Path, comp: &str) {
    let project_name = base.file_name().unwrap().to_string_lossy().to_string();
    if is_installed(&["python3", "python"]).is_none() {
        println!("✖ Python missing, skipping Django.");
        return;
    }
    if Command::new("django-admin")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_err()
    {
        let ans = prompt_input("Django not found. Install now? (y/N): ");
        if ans.to_lowercase() == "y" {
            if let Some(py) = is_installed(&["python3", "python"]) {
                println!("→ {} -m pip install django", py);
                let _ = Command::new(py)
                    .args(&["-m", "pip", "install", "django"])
                    .status();
            }
        } else {
            println!("Skipping Django scaffold.");
            return;
        }
    }
    println!("→ django-admin startproject {} in {:?}", project_name, base);
    let _ = Command::new("django-admin")
        .args(&["startproject", &project_name])
        .current_dir(base)
        .status();

    let src = base.join(&project_name);
    let dst = base.join(comp);
    if dst.exists() {
        println!(
            "✖ Target folder {:?} exists, cannot rename Django project.",
            dst
        );
        return;
    }
    println!("→ renaming {:?} to {:?}", src, dst);
    fs::rename(&src, &dst).expect("Failed to rename Django folder");
    println!("Scaffolded Django in {:?}", dst);
}

fn scaffold_flask_fastapi(base: &Path, comp: &str, tech: &str) {
    let py = match is_installed(&["python3", "python"]) {
        Some(p) => p,
        None => {
            println!("✖ Python missing, skip {}.", tech);
            return;
        }
    };

    let comp_path = base.join(comp);
    fs::create_dir_all(&comp_path).expect("Failed to create component directory");

    // For simplicity we'll use virtualenv (pyenv not implemented here)
    let venv_dir = comp_path.join("venv");
    println!("→ {} -m venv {:?}", py, venv_dir);
    let _ = Command::new(&py)
        .args(&["-m", "venv", venv_dir.to_str().unwrap()])
        .status();

    let package = if tech == "flask" { "flask" } else { "fastapi" };
    let ans = prompt_input(&format!(
        "Ensure package '{}' is installed? (install if missing) (Y/n): ",
        package
    ));
    if ans.to_lowercase() == "y" || ans.is_empty() {
        println!("→ {} -m pip install {}", py, package);
        let _ = Command::new(&py)
            .args(&["-m", "pip", "install", package])
            .status();
    }
    let stub_name = if tech == "flask" { "app.py" } else { "main.py" };
    let stub_content = if tech == "flask" {
        "from flask import Flask
app = Flask(__name__)
@app.route('/')
def home():
    return 'Hello, Flask!'"
    } else {
        "from fastapi import FastAPI
app = FastAPI()
@app.get('/')
async def read_root():
    return {'msg': 'Hello, FastAPI!'}"
    };
    let stub_path = comp_path.join(stub_name);
    fs::write(&stub_path, stub_content).expect("Failed to write stub");
    println!("→ stub {} written", stub_name);
}

fn scaffold_ml_lib(base: &Path, comp: &str, tech: &str) {
    let py = match is_installed(&["python3", "python"]) {
        Some(p) => p,
        None => {
            println!("✖ Python missing, skip {}.", tech);
            return;
        }
    };

    let comp_path = base.join(comp);
    fs::create_dir_all(&comp_path).expect("Failed to create component directory");

    let ans = prompt_input(&format!(
        "Ensure ML package '{}' is installed? (install if missing) (Y/n): ",
        tech
    ));
    if ans.to_lowercase() == "y" || ans.is_empty() {
        println!("→ {} -m pip install {}", py, tech);
        let _ = Command::new(&py)
            .args(&["-m", "pip", "install", tech])
            .status();
    }
    let nb_path = comp_path.join("notebook.ipynb");
    fs::write(
        &nb_path,
        r#"{"cells":[],"metadata":{},"nbformat":4,"nbformat_minor":2}"#,
    )
    .expect("Failed to create notebook");
    println!("→ empty notebook {:?} created", nb_path);
}

fn scaffold_rust(base: &Path, comp: &str) {
    println!("→ cargo new {} in {:?}", comp, base);
    let _ = Command::new("cargo")
        .args(&["new", comp])
        .current_dir(base)
        .status();
}

fn scaffold_js_framework(base: &Path, comp: &str, tech: &str) {
    if is_installed(&["node"]).is_none() {
        println!("✖ Node missing, skip {}.", tech);
        return;
    }
    let template = match tech {
        "react" => "react-ts",
        "vue" => "vue",
        "svelte" => "svelte",
        _ => {
            println!("Unknown JS framework: {}", tech);
            return;
        }
    };
    println!(
        "→ npm create vite@latest {} -- --template {}",
        comp, template
    );
    let _ = Command::new("npm")
        .args(&["create", "vite@latest", comp, "--", "--template", template])
        .current_dir(base)
        .status();
}

fn scaffold_node_init(base: &Path, comp: &str) {
    if is_installed(&["node"]).is_none() {
        println!("✖ Node missing, skip node init.");
        return;
    }
    let comp_path = base.join(comp);
    fs::create_dir_all(&comp_path).expect("Failed to create component directory");
    println!("→ npm init -y in {}", comp);
    let _ = Command::new("npm")
        .args(&["init", "-y"])
        .current_dir(&comp_path)
        .status();
}

fn scaffold_plain_folder(base: &Path, comp: &str, tech: &str) {
    let folder = base.join(format!("{}_{}", comp, tech));
    fs::create_dir_all(&folder).expect("Failed to create folder");
    println!("→ created folder {:?}", folder.file_name().unwrap());
}

fn scaffold_component(base: &Path, comp: &str, tech: &str) {
    let tech_key = tech.to_lowercase();
    match tech_key.as_str() {
        "django" => scaffold_django(base, comp),
        "flask" | "fastapi" => scaffold_flask_fastapi(base, comp, &tech_key),
        "tensorflow" | "torch" | "scikit-learn" => scaffold_ml_lib(base, comp, &tech_key),
        "rust" => scaffold_rust(base, comp),
        "react" | "vue" | "svelte" => scaffold_js_framework(base, comp, &tech_key),
        "node" => scaffold_node_init(base, comp),
        _ => scaffold_plain_folder(base, comp, &tech_key),
    }
}

fn gather_components(proj_dir: &Path) -> Vec<String> {
    let mut techs = Vec::new();
    loop {
        let comp = prompt_input("Component (frontend/backend/etc; blank to finish): ");
        if comp.is_empty() {
            break;
        }
        let tech = prompt_input(&format!("Tech for '{}': ", comp)).to_lowercase();
        techs.push(tech.clone());
        // For basic installer check for JS frameworks:
        if ["react", "angular", "vue", "ember", "svelte"].contains(&tech.as_str()) {
            if is_installed(&["node"]).is_none() {
                println!("✖ Node missing → can't scaffold JS.");
                let comp_path = proj_dir.join(&comp);
                fs::create_dir_all(&comp_path).expect("Failed to create component directory");
                continue;
            }
        }
        scaffold_component(proj_dir, &comp, &tech);
    }
    techs
}

fn scaffold_db(proj_dir: &Path) {
    let db = prompt_input("Add DB (sqlite/postgres/mysql; blank to skip): ").to_lowercase();
    if db == "sqlite" {
        let db_path = proj_dir.join("db.sqlite3");
        fs::File::create(&db_path).expect("Failed to create SQLite file");
        println!("→ Created SQLite file {:?}", db_path.file_name().unwrap());
    } else if db == "postgres" || db == "mysql" {
        let image = if db == "postgres" {
            "postgres:14"
        } else {
            "mysql:8"
        };
        let dc_content = format!(
            r#"version: '3.8'
services:
  db:
    image: {}
    environment:
      - POSTGRES_USER=app
      - POSTGRES_PASSWORD=secret
      - POSTGRES_DB={}
    ports:
      - "5432:5432""#,
            image,
            proj_dir.file_name().unwrap().to_string_lossy()
        );
        let dc_path = proj_dir.join("docker-compose.yml");
        fs::write(&dc_path, dc_content).expect("Failed to write docker-compose.yml");
        println!("→ Generated docker-compose.yml for {}", db);
    }
}

fn write_gitignore(proj_dir: &Path, techs: &[String]) {
    // common ignore chunks
    let common = vec![".vscode/", ".DS_Store", "*.log", ".env", ".env.*"];
    let python = vec![
        "__pycache__/",
        ".pytest_cache/",
        ".mypy_cache/",
        ".venv/",
        "build/",
        "dist/",
        "*.egg-info/",
    ];
    let node = vec![
        "node_modules/",
        ".npm/",
        ".nvm/",
        "dist/",
        "build/",
        ".cache/",
    ];
    let java = vec![
        "target/",
        "*.class",
        "*.jar",
        "*.war",
        "*.ear",
        ".settings/",
        ".project",
        ".classpath",
    ];
    let ruby = vec![".bundle/", "log/", "tmp/", "vendor/bundle/"];
    let go = vec!["bin/", "pkg/", "vendor/"];
    let rust = vec!["target/", "Cargo.lock"];

    let mut ignores = common.join("\n");
    for t in techs {
        match t.as_str() {
            "django" | "flask" | "fastapi" | "pyramid" => {
                ignores.push_str("\n");
                ignores.push_str(&python.join("\n"));
            }
            "node" | "react" | "angular" | "vue" | "ember" | "svelte" => {
                ignores.push_str("\n");
                ignores.push_str(&node.join("\n"));
            }
            "java" => {
                ignores.push_str("\n");
                ignores.push_str(&java.join("\n"));
            }
            "ruby" => {
                ignores.push_str("\n");
                ignores.push_str(&ruby.join("\n"));
            }
            "go" => {
                ignores.push_str("\n");
                ignores.push_str(&go.join("\n"));
            }
            "rust" => {
                ignores.push_str("\n");
                ignores.push_str(&rust.join("\n"));
            }
            _ => {}
        }
    }
    let gitignore_path = proj_dir.join(".gitignore");
    fs::write(&gitignore_path, ignores).expect("Failed to write .gitignore");
    println!("→ Created .gitignore at {:?}", gitignore_path);
}

fn write_readme(proj_dir: &Path, project: &str, techs: &[String]) {
    let content = format!("# {}\n\nTechnologies used: {}", project, techs.join(", "));
    let readme_path = proj_dir.join("README.md");
    fs::write(&readme_path, content).expect("Failed to write README.md");
    println!("→ Created README.md");
}

fn init_git(proj_dir: &Path, project: &str, techs: &[String]) {
    let ans = prompt_input("Initialize a new git repository at project root? (Y/n): ");
    if ans.to_lowercase() == "y" || ans.is_empty() {
        write_gitignore(proj_dir, techs);
        write_readme(proj_dir, project, techs);
        let _ = Command::new("git")
            .arg("init")
            .current_dir(proj_dir)
            .status();
        let _ = Command::new("git")
            .args(&["add", "."])
            .current_dir(proj_dir)
            .status();
        let _ = Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
            .current_dir(proj_dir)
            .status();
        println!("Initialized git repo at {:?}", proj_dir);
    }
}

fn main() {
    let project = prompt_input("Project name: ");
    if project.is_empty() {
        println!("Project name can’t be empty.");
        return;
    }
    let base_input = prompt_input("Base path (default cwd): ");
    let base = if base_input.is_empty() {
        env::current_dir().expect("Failed to get current directory")
    } else {
        let p = env::current_dir().unwrap().join(&base_input);
        if !p.exists() {
            println!("Base path '{}' does not exist. Creating...", base_input);
            fs::create_dir_all(&p).expect("Failed to create base path");
        }
        p
    };

    let proj_dir = base.join(&project);
    if prompt_clone(&proj_dir) {
        return;
    }
    fs::create_dir_all(&proj_dir).expect("Failed to create project directory");
    println!("Created project dir at {:?}", proj_dir);

    let techs = gather_components(&proj_dir);
    scaffold_db(&proj_dir);
    init_git(&proj_dir, &project, &techs);

    println!("🎉 Done scaffolding.");
}
