use super::cli::Skyull;
use crate::modules::cli::templates::template_struct::Template;
use std::env;
use std::process::Stdio;
use std::{fs, process::Command};

pub fn create_project(project_data: Skyull) -> std::io::Result<()> {
    let binding = env::current_dir()?;
    let path_to_create = binding.to_str().unwrap();
    let _path_dir_ = format!("{}\\{}", path_to_create, project_data.name.as_str());
    let path_dir = _path_dir_.as_str();

    if fs::metadata(path_dir).is_ok() {
        println!("Prject already exists\nAdding Dependecies...");
        add_project_dependencies(path_dir, path_to_create);
    } else {
        fs::create_dir(path_dir);
        init_cargo(path_dir);
        add_project_dependencies(path_dir, path_to_create);
    }
    Ok(())
}

fn init_cargo(path_dir: &str) -> std::io::Result<()> {
    Command::new("cargo")
        .stdout(Stdio::null())
        .arg("init")
        .current_dir(path_dir)
        .output()
        .expect("Error creating project cargo binary");
    Ok(())
}

fn add_project_dependencies(path_dir: &str, path_root: &str) -> std::io::Result<()> {
    let _cargo_toml_path = format!("{}\\Cargo.toml", path_dir);
    let _template_path = format!("{}\\templates\\{}.json", path_root, "t1_rocket");
    let template = helper_get_template(_template_path)?;
    println!("{:?}", template);
    Ok(())
}

fn helper_get_template(template_path: String) -> std::io::Result<Template> {
    let _template_content = fs::read_to_string(template_path)?;
    let template: Template = serde_json::from_str(&_template_content)?;
    Ok(template)
}

// fn editing_structures(path_dir: String) -> std::io::Result<()> {
//     Ok(())
// }
