use owo_colors::OwoColorize;

use super::cli::Skyull;
use crate::modules::cli::templates::template_struct::Template;
use std::env;
use std::path::PathBuf;
use std::process::Stdio;
use std::{fs, process::Command};

pub fn create_project(project_data: Skyull) -> std::io::Result<()> {
    let binding = env::current_dir()?;
    let mut path_dir = PathBuf::from(&binding);
    path_dir.push(&project_data.name);
    match fs::metadata(&path_dir) {
        Ok(_) => {
            println!("{}", "Prject already exists".red());
            add_project_dependencies(&path_dir, &binding);
        }
        Err(_) => {
            fs::create_dir(&path_dir);
            init_cargo(&path_dir);
            add_project_dependencies(&path_dir, &binding);
        }
    }
    Ok(())
}

fn init_cargo(path_dir: &PathBuf) -> std::io::Result<()> {
    println!("{}", "Creating bin directory".purple());
    Command::new("cargo")
        .stdout(Stdio::null())
        .arg("init")
        .current_dir(path_dir)
        .output()
        .expect("Error creating project cargo binary");
    Ok(())
}

fn add_project_dependencies(path_dir: &PathBuf, path_root: &PathBuf) -> std::io::Result<()> {
    println!("{}", "Adding project dependencies".cyan());
    let mut cargo_project_dir = PathBuf::from(path_dir);
    cargo_project_dir.push("Cargo.toml");
    let mut path_template = PathBuf::from(path_root);
    path_template.push("templates/t1_rocket.json");
    let template = helper_get_template(&path_template)?;
    println!("{:?}", template);
    Ok(())
}

fn helper_get_template(template_path: &PathBuf) -> std::io::Result<Template> {
    let _template_content = fs::read_to_string(template_path)?;
    let template: Template = serde_json::from_str(&_template_content)?;
    Ok(template)
}

// fn editing_structures(path_dir: String) -> std::io::Result<()> {
//     Ok(())
// }
