use super::templates::templates::get_templates;
use crate::modules::cli::cli::{ArgTemplate, Skyull};
use crate::modules::cli::commands::new::templates::template_struct::{Dependency, Templates};
use owo_colors::OwoColorize;
use std::{
    env, fs,
    path::PathBuf,
    process::{exit, Command, Stdio},
};

pub fn new_project(project_data: Skyull) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let mut project_dir = PathBuf::from(&current_dir);
    project_dir.push(&project_data.name);
    let templates = get_templates();
    match fs::metadata(&project_dir) {
        Ok(_) => {
            println!("{}", "Prject already exists".red());
            exit(0x0);
        }
        Err(_) => {
            fs::create_dir(&project_dir);
            init_cargo(&project_dir);
            add_project_dependencies(&project_dir, &project_data.template, templates);
        }
    }
    Ok(())
}

fn init_cargo(project_dir: &PathBuf) -> std::io::Result<()> {
    println!("{}", "Creating bin directory".cyan());
    Command::new("cargo")
        .stdout(Stdio::null())
        .arg("init")
        .current_dir(project_dir)
        .output()
        .expect("Error creating project cargo binary");
    Ok(())
}

fn add_project_dependencies(project_dir: &PathBuf, template_type: &ArgTemplate, templates: Templates) -> std::io::Result<()> {
    println!("{}", "Adding project dependencies".cyan());
    let common_dependencies = templates.common_dependencies.dependencies;
    let mut dependencies = match template_type {
        ArgTemplate::Rocket => templates.rocket.dependencies,
        ArgTemplate::ActixWeb => templates.actix_web.dependencies,
    };
    dependencies.extend(common_dependencies);
    add_dependencies(&dependencies, project_dir);
    Ok(())
}

fn add_dependencies(dependencies: &Vec<Dependency>, project_dir: &PathBuf) {
    for dep in dependencies.iter() {
        let mut dependency_project = format!("{}", dep.name);
        match &dep.version {
            Some(version) => {
                if !version.is_empty() {
                    dependency_project.push_str(format!("@{}", version).as_str())
                }
            }

            None => (),
        };
        match &dep.features {
            Some(features) => {
                if !features.is_empty() {
                    dependency_project.push_str(format!(" --features {}", features.join(" ")).as_str())
                }
            }
            None => (),
        };
        println!("{}", dependency_project);
        Command::new("cargo")
            .stdout(Stdio::null())
            .arg("add")
            .arg(&dependency_project)
            .current_dir(project_dir)
            .output()
            .expect("Error Adding dependencies");
    }
}
