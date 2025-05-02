mod create;
mod delete;
mod status;
mod update;

use crate::create::{create_project, questionaire};
use crate::delete::delete_project;
use crate::status::project_status;
use crate::update::update_project;

use clap::Parser;
use std::path::Path;

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
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    let path_string = path.display().to_string();
    if args.mode == "default" {
        if args.id != "000" {
            project_status(args.id);
        } else if args.name != "Project" {
            create_project(args.name, args.path);
        } else {
            questionaire();
        }
    } else if args.mode == "create" {
        create_project(args.name, path_string);
    } else if args.mode == "update" {
        if args.id == "000" {
            create_project(args.name, path_string);
        } else {
            update_project(args.name, args.id);
        }
    } else if args.mode == "delete" {
        if args.id == "000" {
            println!("Invalid project ID!");
        } else {
            delete_project(args.name, args.id);
        }
    } else if args.mode == "status" {
        project_status(args.id);
    } else {
        println!("Invalid mode!");
    }
}
