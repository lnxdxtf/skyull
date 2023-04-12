use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Templates {
    pub rocket: WrapperDependencies,
    pub actix_web: WrapperDependencies,
    pub common_dependencies: WrapperDependencies,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WrapperDependencies {
    pub dependencies: Vec<Dependency>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub features: Option<Vec<String>>,
}
