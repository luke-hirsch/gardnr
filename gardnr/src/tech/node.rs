use crate::utils::is_installed;
use std::{fs, io, path::PathBuf, process::Command};

pub fn scaffold_js_project(
    project_dir: &PathBuf,
    component_name: &str,
    tech: &str,
) -> io::Result<()> {
    let component_dir = project_dir.join(component_name);

    // Check if Node.js/npm is installed
    let npm_cmd = match check_node() {
        Some(cmd) => {
            println!("→ Found Node.js/npm: {}", cmd);
            cmd
        }
        None => {
            eprintln!("⚠️  Node.js/npm not found! Creating basic directory structure only.");
            eprintln!("   Install Node.js to enable full scaffolding.");
            fs::create_dir_all(&component_dir)?;
            return Ok(());
        }
    };

    match tech.to_lowercase().as_str() {
        "react" => scaffold_react_with_vite(project_dir, component_name, &npm_cmd),
        "vue" => scaffold_vue_with_vite(project_dir, component_name, &npm_cmd),
        "svelte" => scaffold_svelte_with_vite(project_dir, component_name, &npm_cmd),
        "node" | "nodejs" => scaffold_node_express(&component_dir, &npm_cmd),
        "express" => scaffold_node_express(&component_dir, &npm_cmd),
        "nextjs" | "next" => scaffold_nextjs(project_dir, component_name, &npm_cmd),
        "nuxt" => scaffold_nuxt(project_dir, component_name, &npm_cmd),
        _ => scaffold_generic_node(&component_dir, &npm_cmd, tech),
    }
}

fn check_node() -> Option<String> {
    // Check for npm first (most common)
    if let Some(_) = is_installed(&["npm"]) {
        return Some("npm".to_string());
    }
    // Check for yarn
    if let Some(_) = is_installed(&["yarn"]) {
        return Some("yarn".to_string());
    }
    // Check for pnpm
    if let Some(_) = is_installed(&["pnpm"]) {
        return Some("pnpm".to_string());
    }
    None
}

fn scaffold_react_with_vite(
    project_dir: &PathBuf,
    component_name: &str,
    npm_cmd: &str,
) -> io::Result<()> {
    println!("→ Scaffolding React application with Vite");

    let project_name = project_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("my-react-app");

    println!(
        "→ Running npm create vite@latest {} -- --template react-ts in {}",
        project_name,
        project_dir.display()
    );

    let status = Command::new(npm_cmd)
        .args([
            "create",
            "vite@latest",
            project_name,
            "--",
            "--template",
            "react-ts",
        ])
        .current_dir(project_dir)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "npm create vite failed for React",
        ));
    }

    // Rename the Vite project directory to the component name
    let vite_project_dir = project_dir.join(project_name);
    let component_dir = project_dir.join(component_name);

    if vite_project_dir.exists() && vite_project_dir != component_dir {
        std::fs::rename(&vite_project_dir, &component_dir)?;
        println!(
            "→ Renamed React project directory from '{}' to '{}'",
            project_name, component_name
        );
    }

    // Install dependencies
    println!("→ Installing dependencies...");
    let install_status = Command::new(npm_cmd)
        .arg("install")
        .current_dir(&component_dir)
        .status()?;

    if !install_status.success() {
        eprintln!("⚠️  Failed to install dependencies, but project structure is ready");
    }

    // Update package.json with correct name
    update_package_json(&component_dir, project_name)?;

    println!(
        "→ React + TypeScript project '{}' scaffolded successfully as '{}'",
        project_name, component_name
    );
    println!("   - Run: cd {} && npm run dev", component_name);
    println!("   - Build: cd {} && npm run build", component_name);

    Ok(())
}

fn scaffold_vue_with_vite(
    project_dir: &PathBuf,
    component_name: &str,
    npm_cmd: &str,
) -> io::Result<()> {
    println!("→ Scaffolding Vue.js application with Vite");

    let project_name = project_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("my-vue-app");

    println!(
        "→ Running npm create vue@latest {} -- --typescript --router --pinia --vitest --eslint in {}",
        project_name,
        project_dir.display()
    );

    // Create Vue project with recommended settings (non-interactive)
    let status = Command::new(npm_cmd)
        .args([
            "create",
            "vue@latest",
            project_name,
            "--",
            "--typescript",
            "--router",
            "--pinia",
            "--vitest",
            "--eslint",
        ])
        .current_dir(project_dir)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "npm create vue failed",
        ));
    }

    // Rename the Vue project directory to the component name
    let vue_project_dir = project_dir.join(project_name);
    let component_dir = project_dir.join(component_name);

    if vue_project_dir.exists() && vue_project_dir != component_dir {
        std::fs::rename(&vue_project_dir, &component_dir)?;
        println!(
            "→ Renamed Vue project directory from '{}' to '{}'",
            project_name, component_name
        );
    }

    // Install dependencies
    println!("→ Installing dependencies...");
    let install_status = Command::new(npm_cmd)
        .arg("install")
        .current_dir(&component_dir)
        .status()?;

    if !install_status.success() {
        eprintln!("⚠️  Failed to install dependencies, but project structure is ready");
    }

    // Update package.json with correct name
    update_package_json(&component_dir, project_name)?;

    println!(
        "→ Vue.js project '{}' scaffolded successfully as '{}'",
        project_name, component_name
    );
    println!("   - Run: cd {} && npm run dev", component_name);
    println!("   - Build: cd {} && npm run build", component_name);

    Ok(())
}

fn scaffold_svelte_with_vite(
    project_dir: &PathBuf,
    component_name: &str,
    npm_cmd: &str,
) -> io::Result<()> {
    println!("→ Scaffolding Svelte application with Vite");

    let project_name = project_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("my-svelte-app");

    println!(
        "→ Running npm create svelte@latest {} -- --template skeleton --types typescript --prettier --eslint in {}",
        project_name,
        project_dir.display()
    );

    let status = Command::new(npm_cmd)
        .args([
            "create",
            "svelte@latest",
            project_name,
            "--",
            "--template",
            "skeleton",
            "--types",
            "typescript",
            "--prettier",
            "--eslint",
        ])
        .current_dir(project_dir)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "npm create svelte failed",
        ));
    }

    // Rename the Svelte project directory to the component name
    let svelte_project_dir = project_dir.join(project_name);
    let component_dir = project_dir.join(component_name);

    if svelte_project_dir.exists() && svelte_project_dir != component_dir {
        std::fs::rename(&svelte_project_dir, &component_dir)?;
        println!(
            "→ Renamed Svelte project directory from '{}' to '{}'",
            project_name, component_name
        );
    }

    // Install dependencies
    println!("→ Installing dependencies...");
    let install_status = Command::new(npm_cmd)
        .arg("install")
        .current_dir(&component_dir)
        .status()?;

    if !install_status.success() {
        eprintln!("⚠️  Failed to install dependencies, but project structure is ready");
    }

    // Update package.json with correct name
    update_package_json(&component_dir, project_name)?;

    println!(
        "→ Svelte project '{}' scaffolded successfully as '{}'",
        project_name, component_name
    );
    println!("   - Run: cd {} && npm run dev", component_name);
    println!("   - Build: cd {} && npm run build", component_name);

    Ok(())
}

fn scaffold_nextjs(project_dir: &PathBuf, component_name: &str, _npm_cmd: &str) -> io::Result<()> {
    println!("→ Scaffolding Next.js application");

    let project_name = project_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("my-next-app");

    println!(
        "→ Running npx create-next-app@latest {} --typescript --tailwind --eslint --app --src-dir in {}",
        project_name,
        project_dir.display()
    );

    let status = Command::new("npx")
        .args([
            "create-next-app@latest",
            project_name,
            "--typescript",
            "--tailwind",
            "--eslint",
            "--app",
            "--src-dir",
            "--import-alias",
            "@/*",
        ])
        .current_dir(project_dir)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "create-next-app failed",
        ));
    }

    // Rename the Next.js project directory to the component name
    let next_project_dir = project_dir.join(project_name);
    let component_dir = project_dir.join(component_name);

    if next_project_dir.exists() && next_project_dir != component_dir {
        std::fs::rename(&next_project_dir, &component_dir)?;
        println!(
            "→ Renamed Next.js project directory from '{}' to '{}'",
            project_name, component_name
        );
    }

    println!(
        "→ Next.js project '{}' scaffolded successfully as '{}'",
        project_name, component_name
    );
    println!("   - Run: cd {} && npm run dev", component_name);
    println!("   - Build: cd {} && npm run build", component_name);

    Ok(())
}

fn scaffold_nuxt(project_dir: &PathBuf, component_name: &str, npm_cmd: &str) -> io::Result<()> {
    println!("→ Scaffolding Nuxt.js application");

    let project_name = project_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("my-nuxt-app");

    println!(
        "→ Running npx nuxi@latest init {} in {}",
        project_name,
        project_dir.display()
    );

    let status = Command::new("npx")
        .args(["nuxi@latest", "init", project_name])
        .current_dir(project_dir)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "nuxi init failed"));
    }

    // Rename the Nuxt project directory to the component name
    let nuxt_project_dir = project_dir.join(project_name);
    let component_dir = project_dir.join(component_name);

    if nuxt_project_dir.exists() && nuxt_project_dir != component_dir {
        std::fs::rename(&nuxt_project_dir, &component_dir)?;
        println!(
            "→ Renamed Nuxt project directory from '{}' to '{}'",
            project_name, component_name
        );
    }

    // Install dependencies
    println!("→ Installing dependencies...");
    let install_status = Command::new(npm_cmd)
        .arg("install")
        .current_dir(&component_dir)
        .status()?;

    if !install_status.success() {
        eprintln!("⚠️  Failed to install dependencies, but project structure is ready");
    }

    println!(
        "→ Nuxt.js project '{}' scaffolded successfully as '{}'",
        project_name, component_name
    );
    println!("   - Run: cd {} && npm run dev", component_name);
    println!("   - Build: cd {} && npm run build", component_name);

    Ok(())
}

fn scaffold_node_express(component_dir: &PathBuf, npm_cmd: &str) -> io::Result<()> {
    println!("→ Scaffolding Node.js Express application");

    fs::create_dir_all(component_dir)?;

    // Initialize package.json
    let status = Command::new(npm_cmd)
        .args(["init", "-y"])
        .current_dir(component_dir)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "npm init failed"));
    }

    // Install Express
    println!("→ Installing Express...");
    let install_status = Command::new(npm_cmd)
        .args(["install", "express", "cors", "helmet", "dotenv"])
        .current_dir(component_dir)
        .status()?;

    if !install_status.success() {
        eprintln!("⚠️  Failed to install Express dependencies");
    }

    // Install dev dependencies
    let _dev_install = Command::new(npm_cmd)
        .args([
            "install",
            "--save-dev",
            "nodemon",
            "@types/node",
            "@types/express",
        ])
        .current_dir(component_dir)
        .status();

    // Create server.js
    let server_js = component_dir.join("server.js");
    let server_code = r#"const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(helmet());
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Routes
app.get('/', (req, res) => {
    res.json({
        message: 'Hello from Express!',
        timestamp: new Date().toISOString()
    });
});

app.get('/health', (req, res) => {
    res.json({ status: 'healthy' });
});

app.get('/api/status', (req, res) => {
    res.json({
        status: 'running',
        version: '1.0.0',
        environment: process.env.NODE_ENV || 'development'
    });
});

// Error handling middleware
app.use((err, req, res, next) => {
    console.error(err.stack);
    res.status(500).json({ error: 'Something went wrong!' });
});

// 404 handler
app.use((req, res) => {
    res.status(404).json({ error: 'Route not found' });
});

app.listen(PORT, () => {
    console.log(`🚀 Server running on http://localhost:${PORT}`);
    console.log(`📚 Environment: ${process.env.NODE_ENV || 'development'}`);
});

module.exports = app;
"#;

    fs::write(server_js, server_code)?;

    // Create .env file
    let env_file = component_dir.join(".env");
    let env_content = "NODE_ENV=development\nPORT=3000\n";
    fs::write(env_file, env_content)?;

    // Update package.json scripts
    update_package_json_scripts(component_dir)?;

    println!("→ Express application scaffolded");
    println!("   Files: server.js, .env, package.json");
    println!(
        "   - Run: cd {} && npm run dev",
        component_dir.file_name().unwrap().to_str().unwrap()
    );

    Ok(())
}

fn scaffold_generic_node(component_dir: &PathBuf, npm_cmd: &str, tech: &str) -> io::Result<()> {
    println!("→ Scaffolding generic Node.js project for '{}'", tech);

    fs::create_dir_all(component_dir)?;

    // Initialize package.json
    let status = Command::new(npm_cmd)
        .args(["init", "-y"])
        .current_dir(component_dir)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "npm init failed"));
    }

    // Create index.js
    let index_js = component_dir.join("index.js");
    let index_code = format!(
        r#"#!/usr/bin/env node

/**
 * {} - Node.js Application
 */

console.log('Hello from {}!');

// Add your {} code here
function main() {{
    console.log('Starting {} application...');
    // Your application logic here
}}

if (require.main === module) {{
    main();
}}

module.exports = {{ main }};
"#,
        tech, tech, tech, tech
    );

    fs::write(index_js, index_code)?;

    // Try to install the package if it might be a Node.js package
    if is_likely_node_package(tech) {
        println!("→ Attempting to install {} package...", tech);
        let _install_result = Command::new(npm_cmd)
            .args(["install", tech])
            .current_dir(component_dir)
            .status();
    }

    println!("→ Generic Node.js project scaffolded");
    println!("   Files: index.js, package.json");

    Ok(())
}

fn is_likely_node_package(tech: &str) -> bool {
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
    ];

    node_packages
        .iter()
        .any(|&pkg| tech.to_lowercase().contains(pkg))
}

fn update_package_json(component_dir: &PathBuf, project_name: &str) -> io::Result<()> {
    let package_json_path = component_dir.join("package.json");

    if !package_json_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&package_json_path)?;

    // Simple string replacement for the name field
    let updated_content = content.replace(
        &format!("\"name\": \"{}\"", project_name),
        &format!("\"name\": \"{}\"", project_name),
    );

    fs::write(package_json_path, updated_content)?;

    Ok(())
}

fn update_package_json_scripts(component_dir: &PathBuf) -> io::Result<()> {
    let package_json_path = component_dir.join("package.json");

    if !package_json_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&package_json_path)?;

    // Add development scripts
    let scripts_addition = r#"  "scripts": {
    "start": "node server.js",
    "dev": "nodemon server.js",
    "test": "echo \"Error: no test specified\" && exit 1"
  },"#;

    let updated_content = content.replace(
        "  \"scripts\": {\n    \"test\": \"echo \\\"Error: no test specified\\\" && exit 1\"\n  },",
        scripts_addition,
    );

    fs::write(package_json_path, updated_content)?;

    Ok(())
}
