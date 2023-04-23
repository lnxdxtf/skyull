use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
    id: String,
    name: Option<String>,
    role: Option<UserRole>,
    image: Option<String>,
}

impl UserModel {
    pub fn new(id: String, name: Option<String>, role: Option<UserRole>, image: Option<String>) -> UserModel {
        UserModel { id, name, role, image }
    }
}

pub enum UserRole {
    Admin,
    Normal,
    Premium,
}
