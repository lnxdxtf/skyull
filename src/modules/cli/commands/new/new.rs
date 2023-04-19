use owo_colors::OwoColorize;

use super::add_dependencies::add_project_dependencies;
use super::templates::templates::get_templates;
use crate::modules::cli::{
    analyze::indicator::{indicator, COMMAND_NEW_STAGES},
    cli::Skyull,
};
use std::{
    env, fs,
    path::PathBuf,
    process::{exit, Command, Stdio},
};

/// Check if the directory exists and exits with 0x0
/// else starts a new directory and add dependencies
pub fn new_project(project_data: Skyull) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let mut project_dir = PathBuf::from(&current_dir);
    project_dir.push(&project_data.name);
    let binding_indicator = indicator();
    let indicator = binding_indicator.lock().unwrap();
    let templates = get_templates();
    match fs::metadata(&project_dir) {
        Ok(_) => {
            indicator.finish_with_message("Project with this name already exists in this directory");
            // println!("{}", "Prject already exists".red());
            exit(0x0);
        }
        Err(_) => {
            indicator.set_message(COMMAND_NEW_STAGES[0]);
            fs::create_dir(&project_dir);
            init_cargo(&project_dir);

            indicator.set_message(COMMAND_NEW_STAGES[1]);
            add_project_dependencies(&project_dir, &project_data.template, templates);
        }
    }
    Ok(())
}

/// Start a new project with:
/// ```rust ignore
/// cargo init
/// ```
fn init_cargo(project_dir: &PathBuf) -> std::io::Result<()> {
    Command::new("cargo")
        .stdout(Stdio::null())
        .arg("init")
        .current_dir(project_dir)
        .output()
        .unwrap_or_else(|err| {
            println!("Error {}\nError: {}", COMMAND_NEW_STAGES[0].red(), err.on_red());
            exit(0x0)
        });
    Ok(())
}
