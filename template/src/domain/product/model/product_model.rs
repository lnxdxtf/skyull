use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductModel {
    id: String,
    name: Option<String>,
    price: Option<f32>,
    description: Option<String>,
    category: Option<String>,
    image: Option<String>,
}

impl ProductModel {
    pub fn new(
        id: String,
        name: Option<String>,
        price: Option<f32>,
        description: Option<String>,
        category: Option<String>,
        image: Option<String>,
    ) -> ProductModel {
        ProductModel {
            id,
            name,
            price,
            description,
            category,
            image,
        }
    }
}
