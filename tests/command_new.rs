#[cfg(test)]
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};

#[test]
fn command_skyull_new() -> std::io::Result<()> {
    let name_project_dir = "skyull_project_test";
    let result = Command::new("cargo")
        .stdout(Stdio::null())
        .arg("run")
        .arg("--")
        .arg("new")
        .arg("--name")
        .arg(name_project_dir)
        .current_dir(env::current_dir()?)
        .output();

    match result {
        Ok(_) => {
            let mut path_dir = PathBuf::from(env::current_dir()?);
            path_dir.push(name_project_dir);
            fs::remove_dir_all(path_dir)?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}
