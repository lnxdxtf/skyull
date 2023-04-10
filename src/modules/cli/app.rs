use crate::modules::cli::{
    cli::{CommandsSkyull, Skyull},
    handlers::create_project,
};

#[allow(unused_must_use)]
pub fn start_skyull() {
    let args = <Skyull as clap::Parser>::parse();

    match args.cmd {
        CommandsSkyull::New => create_project(args),
        CommandsSkyull::Other => todo!(),
    };
}
