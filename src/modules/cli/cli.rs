use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author="lnxdxtf", version="1.0", about, long_about = None)]
pub struct Skyull {
    /// Command that will be executed
    pub command: EnumCommandsSkyull,

    /// Name of the project
    #[arg(short, long, default_value = "skyull_project")]
    pub name: String,

    /// Template to be used to generate templates on the command "new"
    #[arg(short, long, default_value = "rocket")]
    pub template: ArgTemplate,
}

/// Possible Commands
#[derive(ValueEnum, Debug, Clone)]
pub enum EnumCommandsSkyull {
    /// Command that will be executed to generate templates
    New,

    /// Developing...
    Other,
}

/// Possible templates arguments for the command "new"
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
