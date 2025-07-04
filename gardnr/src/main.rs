mod create;
mod delete;
mod status;
mod tech;
mod update;
mod utils;

use crate::create::{Component, Project, create_project, questionnaire};
use crate::delete::delete_project;
use crate::status::project_status;
use crate::update::update_project;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// mode
    #[arg(short, long, default_value = "default")]
    mode: String,

    /// project name
    #[arg(short, long, default_value = "Project")]
    name: String,

    /// path
    #[arg(short, long, default_value = ".")]
    path: String,

    /// project ID
    #[arg(short, long, default_value = "000")]
    id: String,

    /// components
    #[arg(short, long, default_value = "")]
    components: Vec<String>,

    /// component tech
    #[arg(short, long, default_value = "")]
    tech: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut components = Vec::new();

    for (component, tech) in args.components.iter().zip(args.tech.iter()) {
        components.push(Component {
            name: component.clone(),
            tech: tech.clone(),
        });
    }

    let new_project = Project {
        name: args.name,
        path: args.path,
        components: components,
    };

    if args.mode == "default" {
        if args.id != "000" {
            project_status(args.id);
        } else if new_project.name != "Project" {
            create_project(new_project);
        } else {
            questionnaire();
        }
    } else if args.mode == "create" {
        create_project(new_project);
    } else if args.mode == "update" {
        if args.id == "000" {
            create_project(new_project);
        } else {
            update_project(new_project.name, args.id);
        }
    } else if args.mode == "delete" {
        if args.id == "000" {
            println!("Invalid project ID!");
        } else {
            delete_project(new_project.name, args.id);
        }
    } else if args.mode == "status" {
        project_status(args.id);
    } else {
        println!("Invalid mode!");
    }
}
