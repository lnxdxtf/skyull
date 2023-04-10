use super::cli::{ArgTemplate, Skyull};
use crate::modules::cli::templates::template_struct::{Dependency, Template};
use owo_colors::OwoColorize;
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
    vec,
};

pub fn create_project(project_data: Skyull) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let mut project_dir = PathBuf::from(&current_dir);
    project_dir.push(&project_data.name);
    match fs::metadata(&project_dir) {
        Ok(_) => {
            println!("{}", "Prject already exists".red());
            add_project_dependencies(&project_dir, &project_data.template);
        }
        Err(_) => {
            fs::create_dir(&project_dir);
            init_cargo(&project_dir);
            add_project_dependencies(&project_dir, &project_data.template);
        }
    }
    Ok(())
}

fn init_cargo(project_dir: &PathBuf) -> std::io::Result<()> {
    println!("{}", "Creating bin directory".purple());
    Command::new("cargo")
        .stdout(Stdio::null())
        .arg("init")
        .current_dir(project_dir)
        .output()
        .expect("Error creating project cargo binary");
    Ok(())
}

fn add_project_dependencies(project_dir: &PathBuf, template_type: &ArgTemplate) -> std::io::Result<()> {
    println!("{}", "Adding project dependencies".cyan());
    let mut cargo_project_dir = PathBuf::from(project_dir);
    let common_dependencies = common_dependencies();
    let (template, dependencies) = match template_type {
        ArgTemplate::Rocket => {
            let rocket_package_name = env::var("ROCKET_PACKAGE_NAME").unwrap_or_default();
            let rocket_version = env::var("ROCKET_VERSION").ok();
            let rocket_features = match !env::var("ROCKET_FEATURES").unwrap_or_default().is_empty() {
                true => Some(env::var("ROCKET_FEATURES").unwrap_or_default().split(",").map(|s| s.to_string()).collect()),
                false => None,
            };

            let mut dependencies = vec![Dependency::new(rocket_package_name, rocket_version, rocket_features)];
            dependencies.extend(common_dependencies);
            let template = Template::new(None, dependencies.clone());
            (template, dependencies)
        }
        ArgTemplate::Actix => todo!(),
    };
    add_dependency(&dependencies);
    Ok(())
}

fn common_dependencies() -> Vec<Dependency> {
    let prefix_dependencies_vars = vec!["REDIS", "MONGODB"];
    let sub_dependencies_vars = vec!["PACKAGE_NAME", "VERSION", "FEATURES"];
    prefix_dependencies_vars
        .iter()
        .map(|prefix_var| {
            let vars: Vec<String> = sub_dependencies_vars.iter().map(|sub_var| format!("{}_{}", prefix_var, sub_var)).collect();
            let package_name = env::var(&vars[0]).unwrap_or_default();
            let version = env::var(&vars[1]).ok();
            let features: Option<Vec<String>> = match !env::var(&vars[2]).unwrap_or_default().is_empty() {
                true => Some(env::var(&vars[2]).unwrap_or_default().split(",").map(|s| s.to_string()).collect()),
                false => None,
            };
            Dependency::new(package_name, version, features)
        })
        .collect()
}

fn add_dependency(dependencies: &Vec<Dependency>) {
    for dep in dependencies.iter() {
        let mut dependency_project = format!("{}", dep.name);
        match &dep.version {
            Some(version) => dependency_project.push_str(format!("@{}", version).as_str()),
            None => (),
        };
        match &dep.features {
            Some(features) => dependency_project.push_str(format!(" --features {}", features.join(" ")).as_str()),
            None => (),
        };
        println!("{}", dependency_project);
        // Command::new("cargo")
        //     // .stdout(Stdio::null())
        //     .arg("add")
        //     .arg(&dependency_project)
        //     .current_dir(project_dir)
        //     // .output()
        //     .spawn()
        //     .expect("Error Adding dependencies");
        // println!("{:?}\n{:?}\n", project_dir, cargo_project_dir);
    }
}
