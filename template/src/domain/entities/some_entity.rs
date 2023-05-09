use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::traits::base_traits_entities::Getters;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SomeEntity {
    id: Uuid,
    name: String,
    // ...
}

impl Getters for SomeEntity {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
