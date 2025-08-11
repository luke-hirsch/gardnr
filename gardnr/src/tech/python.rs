use crate::utils::{DEFAULT_EXECUTABLES, is_installed};
use std::{fs, io, path::PathBuf, process::Command};

pub fn scaffold_python_project(
    project_dir: &PathBuf,
    component_name: &str,
    tech: &str,
) -> io::Result<()> {
    let component_dir = project_dir.join(component_name);

    // Check if Python is installed
    let python_cmd = match check_python() {
        Some(cmd) => {
            println!("→ Found Python: {}", cmd);
            cmd
        }
        None => {
            eprintln!("⚠️  Python not found! Creating basic directory structure only.");
            eprintln!("   Install Python to enable full scaffolding.");
            fs::create_dir_all(&component_dir)?;
            return Ok(());
        }
    };

    match tech.to_lowercase().as_str() {
        "django" => scaffold_django(&project_dir, component_name, &python_cmd),
        "flask" => scaffold_flask(&component_dir, &python_cmd),
        "fastapi" => scaffold_fastapi(&component_dir, &python_cmd),
        "pyramid" => scaffold_pyramid(&component_dir, &python_cmd),
        _ => scaffold_generic_python(&component_dir, &python_cmd, tech),
    }
}

fn check_python() -> Option<String> {
    is_installed(DEFAULT_EXECUTABLES.python)
}

fn is_package_installed(python_cmd: &str, package: &str) -> bool {
    let output = Command::new(python_cmd)
        .args(["-m", "pip", "show", package])
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn install_package(python_cmd: &str, package: &str) -> io::Result<bool> {
    println!("→ Installing {} with pip...", package);

    let status = Command::new(python_cmd)
        .args(["-m", "pip", "install", package])
        .status()?;

    Ok(status.success())
}

fn prompt_install(package: &str) -> bool {
    println!("Package '{}' is not installed.", package);
    print!("Install it now? (Y/n): ");

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim().to_lowercase();
            input.is_empty() || input == "y" || input == "yes"
        }
        Err(_) => false,
    }
}

fn ensure_package_installed(python_cmd: &str, package: &str) -> io::Result<bool> {
    if is_package_installed(python_cmd, package) {
        println!("→ {} is already installed", package);
        return Ok(true);
    }

    if prompt_install(package) {
        install_package(python_cmd, package)
    } else {
        println!("→ Skipping {} installation", package);
        Ok(false)
    }
}

fn scaffold_django(
    project_dir: &PathBuf,
    component_name: &str,
    python_cmd: &str,
) -> io::Result<()> {
    println!("→ Scaffolding Django project");

    // Check if Django is installed
    if !ensure_package_installed(python_cmd, "django")? {
        println!("→ Creating basic directory without Django scaffolding");
        fs::create_dir_all(&project_dir.join(component_name))?;
        return Ok(());
    }

    // Use the project name for Django project, not the component name
    let project_name = project_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("myproject");

    println!(
        "→ Running django-admin startproject {} in {}",
        project_name,
        project_dir.display()
    );

    let status = Command::new("django-admin")
        .arg("startproject")
        .arg(project_name)
        .current_dir(project_dir)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "django-admin startproject failed",
        ));
    }

    // Rename the Django project directory to the component name
    let django_project_dir = project_dir.join(project_name);
    let component_dir = project_dir.join(component_name);

    if django_project_dir.exists() && django_project_dir != component_dir {
        std::fs::rename(&django_project_dir, &component_dir)?;
        println!(
            "→ Renamed Django project directory from '{}' to '{}'",
            project_name, component_name
        );
    }

    // Create a requirements.txt in the component directory
    let requirements_path = component_dir.join("requirements.txt");
    let requirements_content =
        format!("Django>=4.2,<5.0\ndjango-cors-headers>=4.0\ndjango-environ>=0.10\n");
    fs::write(requirements_path, requirements_content)?;

    println!(
        "→ Django project '{}' scaffolded successfully as '{}'",
        project_name, component_name
    );
    println!(
        "   - Run: cd {} && python manage.py runserver",
        component_name
    );

    Ok(())
}

fn scaffold_flask(component_dir: &PathBuf, python_cmd: &str) -> io::Result<()> {
    println!("→ Scaffolding Flask application");

    fs::create_dir_all(component_dir)?;

    // Check and install Flask
    let flask_installed = ensure_package_installed(python_cmd, "flask")?;

    // Create app.py
    let app_py = component_dir.join("app.py");
    let flask_code = r#"from flask import Flask, jsonify

app = Flask(__name__)

@app.route('/')
def home():
    return jsonify({"message": "Hello, Flask!"})

@app.route('/health')
def health():
    return jsonify({"status": "healthy"})

@app.route('/api/hello')
def api_hello():
    return jsonify({"message": "Hello from Flask API!"})

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)
"#;

    fs::write(app_py, flask_code)?;

    // Create requirements.txt
    let requirements = component_dir.join("requirements.txt");
    let requirements_content = if flask_installed {
        "Flask>=2.3,<3.0\nFlask-CORS>=4.0\npython-dotenv>=1.0\n"
    } else {
        "# Flask not installed - run: pip install -r requirements.txt\nFlask>=2.3,<3.0\nFlask-CORS>=4.0\npython-dotenv>=1.0\n"
    };
    fs::write(requirements, requirements_content)?;

    // Create .env file
    let env_file = component_dir.join(".env");
    let env_content = "FLASK_ENV=development\nFLASK_DEBUG=True\nFLASK_APP=app.py\n";
    fs::write(env_file, env_content)?;

    println!("→ Flask application scaffolded");
    println!("   Files: app.py, requirements.txt, .env");
    if flask_installed {
        println!(
            "   - Run: cd {} && python app.py",
            component_dir.file_name().unwrap().to_str().unwrap()
        );
    }

    Ok(())
}

fn scaffold_fastapi(component_dir: &PathBuf, python_cmd: &str) -> io::Result<()> {
    println!("→ Scaffolding FastAPI application");

    fs::create_dir_all(component_dir)?;

    // Check and install FastAPI and uvicorn
    let fastapi_installed = ensure_package_installed(python_cmd, "fastapi")?;
    let uvicorn_installed = ensure_package_installed(python_cmd, "uvicorn")?;

    // Create main.py
    let main_py = component_dir.join("main.py");
    let fastapi_code = r#"from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI(title="FastAPI App", version="1.0.0")

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.get("/")
async def read_root():
    return {"message": "Hello, FastAPI!"}

@app.get("/health")
async def health_check():
    return {"status": "healthy"}

@app.get("/api/hello")
async def api_hello():
    return {"message": "Hello from FastAPI!"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
"#;

    fs::write(main_py, fastapi_code)?;

    // Create requirements.txt
    let requirements = component_dir.join("requirements.txt");
    let requirements_content = if fastapi_installed && uvicorn_installed {
        "fastapi>=0.104,<1.0\nuvicorn[standard]>=0.24,<1.0\npython-multipart>=0.0.6\n"
    } else {
        "# FastAPI/uvicorn not installed - run: pip install -r requirements.txt\nfastapi>=0.104,<1.0\nuvicorn[standard]>=0.24,<1.0\npython-multipart>=0.0.6\n"
    };
    fs::write(requirements, requirements_content)?;

    println!("→ FastAPI application scaffolded");
    println!("   Files: main.py, requirements.txt");
    if fastapi_installed && uvicorn_installed {
        println!(
            "   - Run: cd {} && python main.py",
            component_dir.file_name().unwrap().to_str().unwrap()
        );
        println!(
            "   - Or: cd {} && uvicorn main:app --reload",
            component_dir.file_name().unwrap().to_str().unwrap()
        );
    }

    Ok(())
}

fn scaffold_pyramid(component_dir: &PathBuf, python_cmd: &str) -> io::Result<()> {
    println!("→ Scaffolding Pyramid application");

    fs::create_dir_all(component_dir)?;

    // Check and install Pyramid
    let pyramid_installed = ensure_package_installed(python_cmd, "pyramid")?;

    // Create app.py
    let app_py = component_dir.join("app.py");
    let pyramid_code = r#"from pyramid.config import Configurator
from pyramid.response import Response
from pyramid.view import view_config
import json

@view_config(route_name='home', renderer='json')
def home_view(request):
    return {'message': 'Hello, Pyramid!'}

@view_config(route_name='health', renderer='json')
def health_view(request):
    return {'status': 'healthy'}

def main():
    config = Configurator()

    config.add_route('home', '/')
    config.add_route('health', '/health')
    config.scan()

    app = config.make_wsgi_app()
    return app

if __name__ == '__main__':
    from wsgiref.simple_server import make_server
    app = main()
    server = make_server('0.0.0.0', 6543, app)
    print('Serving on http://0.0.0.0:6543')
    server.serve_forever()
"#;

    fs::write(app_py, pyramid_code)?;

    // Create requirements.txt
    let requirements = component_dir.join("requirements.txt");
    let requirements_content = if pyramid_installed {
        "pyramid>=2.0\nwaitress>=2.1\n"
    } else {
        "# Pyramid not installed - run: pip install -r requirements.txt\npyramid>=2.0\nwaitress>=2.1\n"
    };
    fs::write(requirements, requirements_content)?;

    println!("→ Pyramid application scaffolded");
    println!("   Files: app.py, requirements.txt");
    if pyramid_installed {
        println!(
            "   - Run: cd {} && python app.py",
            component_dir.file_name().unwrap().to_str().unwrap()
        );
    }

    Ok(())
}

fn scaffold_generic_python(
    component_dir: &PathBuf,
    python_cmd: &str,
    tech: &str,
) -> io::Result<()> {
    println!("→ Scaffolding generic Python project for '{}'", tech);

    fs::create_dir_all(component_dir)?;

    // Check if the specified package exists
    let package_installed = if !tech.is_empty() && tech != "python" {
        ensure_package_installed(python_cmd, tech)?
    } else {
        true
    };

    // Create main.py
    let main_py = component_dir.join("main.py");
    let main_code = format!(
        r#"#!/usr/bin/env python3
"""
{} - Python Application
"""

def main():
    print("Hello from {}!")
    # Add your {} code here

if __name__ == "__main__":
    main()
"#,
        tech, tech, tech
    );

    fs::write(main_py, main_code)?;

    // Create requirements.txt
    let requirements = component_dir.join("requirements.txt");
    let requirements_content = if tech != "python" && !tech.is_empty() {
        if package_installed {
            format!("# Requirements for {}\n{}\n", tech, tech)
        } else {
            format!(
                "# {} not installed - run: pip install -r requirements.txt\n{}\n",
                tech, tech
            )
        }
    } else {
        "# Add your Python dependencies here\n".to_string()
    };
    fs::write(requirements, requirements_content)?;

    // Create __init__.py to make it a package
    let init_py = component_dir.join("__init__.py");
    fs::write(init_py, "")?;

    println!("→ Generic Python project scaffolded");
    println!("   Files: main.py, requirements.txt, __init__.py");

    Ok(())
}
