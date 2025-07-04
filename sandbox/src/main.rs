use std::{io, process::Command};

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

fn main() {
    println!("package?");
    let mut package_name = String::new();
    io::stdin()
        .read_line(&mut package_name)
        .expect("Failed to read project name");
    package_name = package_name.trim().to_string();
    package_installation(&package_name, Option::from(true));
}
