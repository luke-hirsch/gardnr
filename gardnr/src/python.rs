use super::utils::{DEFAULT_EXECUTABLES, is_installed};
use std::path::Path;
use std::process::Command;

pub fn check_python_installed() -> Option<String> {
    is_installed(DEFAULT_EXECUTABLES.python)
}

pub fn check_package_installed(packagename: &str) -> Option<bool> {
    let show_output = Command::new("pip")
        .arg("show")
        .arg(packagename)
        .output()
        .expect("Failed to run pip show");
    if show_output.status.success() {
        Some(true)
    } else {
        Some(false)
    }
}

fn package_installation(packagename: &str, update: Option<bool>) {
    let update = update.unwrap_or(false);
    if update {
        // Install with upgrade
        let output = Command::new("pip")
            .arg("install")
            .arg("--upgrade")
            .arg(packagename)
            .output()
            .expect("Failed to run pip install --upgrade");

        if output.status.success() {
            println!(
                "Package {} upgraded successfully:\n{}",
                packagename,
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            eprintln!(
                "Failed to upgrade package {}:\n{}",
                packagename,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    } else {
        // Check if installed
        let package_installed = package_inscheck_package_installed(packagename);
        if package_installed {
            println!(
                "Package {} is already installed:\n{}",
                packagename,
                String::from_utf8_lossy(&show_output.stdout)
            );
        } else {
            println!("Installing package {}...", packagename);
            let install_output = Command::new("pip")
                .arg("install")
                .arg(packagename)
                .output()
                .expect("Failed to run pip install");

            if install_output.status.success() {
                println!(
                    "Package {} installed successfully:\n{}",
                    packagename,
                    String::from_utf8_lossy(&install_output.stdout)
                );
            } else {
                eprintln!(
                    "Failed to install package {}:\n{}",
                    packagename,
                    String::from_utf8_lossy(&install_output.stderr)
                );
            }
        }
    }
}

pub fn scaffold_django(project_name: &str) {
    // checking if django is installed
    let check_django_installed = check_package_installed("django");
    if !check_django_installed {
        package_installation("django", true)
    }

    // calling django-admin startproject
    let startproject_output = Command::new("django-admin")
        .arg("startproject")
        .arg(project_name)
        .output()
        .expect("Failed to run django-admin startproject");

    if startproject_output.status.success() {
        println!(
            "Django project {} created successfully:\n{}",
            project_name,
            String::from_utf8_lossy(&startproject_output.stdout)
        );
    } else {
        eprintln!(
            "Failed to create Django project {}:\n{}",
            project_name,
            String::from_utf8_lossy(&startproject_output.stderr)
        );
    }
}

// pub fn scaffold_flask(project_name: &str, path: &Path) {
//     // checking if flask is installed
//     let check_flask_installed = check_package_installed("flask");
//     if !check_flask_installed {
//         package_installation("flask", true)
//     }
//     // scaffolding flask project
// }cd

// pub fn scaffold_ml(project_name: &str, path: &Path, lib: &str) {}
