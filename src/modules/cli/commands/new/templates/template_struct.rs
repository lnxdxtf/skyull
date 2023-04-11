use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub package: Option<Package>,
    pub dependencies: Vec<Dependency>,
}
impl Template {
    pub fn new(package: Option<Package>, dependencies: Vec<Dependency>) -> Template {
        Template { package, dependencies }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
    pub edition: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub features: Option<Vec<String>>,
}
impl Dependency {
    pub fn new(name: String, version: Option<String>, features: Option<Vec<String>>) -> Dependency {
        Dependency { name, version, features }
    }
}
