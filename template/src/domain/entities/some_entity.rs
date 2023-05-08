use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SomeEntity {
    name: String,
    // ...
}

impl SomeEntity {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    // ...
}
