use owo_colors::OwoColorize;
use std::{
    path::PathBuf,
    process::{exit, Command, Stdio},
};

use crate::modules::cli::{analyze::indicator::COMMAND_NEW_STAGES, cli::ArgTemplate};

use super::templates::template_struct::{Dependency, Templates};

/// Access the project directory and add the dependencies
pub fn add_project_dependencies(project_dir: &PathBuf, template_type: &ArgTemplate, templates: Templates) -> std::io::Result<()> {
    let common_dependencies = templates.common_dependencies.dependencies;
    let mut dependencies = match template_type {
        ArgTemplate::Rocket => templates.rocket.dependencies,
        ArgTemplate::ActixWeb => templates.actix_web.dependencies,
    };
    dependencies.extend(common_dependencies);
    add_dependencies(&dependencies, project_dir);
    Ok(())
}

/// Add the dependencies:
/// ```rust ignore
/// cargo add <Dependency>@<Version> --features <feature>... --offline
/// ```
fn add_dependencies(dependencies: &Vec<Dependency>, project_dir: &PathBuf) {
    for dep in dependencies.iter() {
        let mut dependency_project = format!("{}", dep.name);
        match !&dep.name.is_empty() {
            true => {
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

                Command::new("cargo")
                    .stdout(Stdio::null())
                    .arg("add")
                    .arg(&dependency_project)
                    .arg("--offline")
                    .current_dir(project_dir)
                    .output()
                    .unwrap_or_else(|err| {
                        println!("Error {} - {}\nERROR:{:?}", COMMAND_NEW_STAGES[1].red(), dependency_project, err.on_red());
                        exit(0x0)
                    });
            }
            false => {
                println!(
                    "{}\n{:?} - Dependency project is empty, remove from array dependencies template",
                    COMMAND_NEW_STAGES[1].yellow(),
                    dependency_project.on_yellow()
                );
                exit(0x0);
            }
        }
    }
}
