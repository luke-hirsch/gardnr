use super::utils::{DEFAULT_EXECUTABLES, is_installed};
use std::path::Path;
use std::process::Command;

pub fn check_python_installed() -> Option<String> {
    is_installed(DEFAULT_EXECUTABLES.python)
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
        let show_output = Command::new("pip")
            .arg("show")
            .arg(packagename)
            .output()
            .expect("Failed to run pip show");
        if show_output.status.success() {
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

pub fn scaffold_django(project_name: &str) {}

pub fn scaffold_flask(project_name: &str, path: &Path) {}

pub fn scaffold_ml(project_name: &str, path: &Path, lib: &str) {}
