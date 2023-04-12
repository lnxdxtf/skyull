use super::commands::new::new::new_project;
use crate::modules::cli::cli::{EnumCommandsSkyull, Skyull};

//// Start Skyull CLI and parsing...
pub fn start_skyull() {
    let args = <Skyull as clap::Parser>::parse();
    match args.command {
        EnumCommandsSkyull::New => new_project(args),
        EnumCommandsSkyull::Other => todo!(),
    };
}
