use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author="lnxdxtf", version="1.0", about, long_about = None)]
pub struct Skyull {
    pub command: EnumCommandsSkyull,
    /// Name of the project
    #[arg(short, long, default_value = "skyull_project")]
    pub name: String,

    #[arg(short, long, default_value = "rocket")]
    pub template: ArgTemplate,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum EnumCommandsSkyull {
    New,
    Other,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ArgTemplate {
    Rocket,
    ActixWeb,
}
impl ToString for ArgTemplate {
    fn to_string(&self) -> String {
        match *self {
            ArgTemplate::Rocket => "Rocket".to_string(),
            ArgTemplate::ActixWeb => "ActixWeb".to_string(),
        }
    }
}
