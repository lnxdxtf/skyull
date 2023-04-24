use crate::domain::product::{model::product_model::ProductModel, repository::product_repository::ProductRepository};

fn mocked_products() -> std::io::Result<Vec<ProductModel>> {
    let mocked_data_as_bytes = include_bytes!("data_product.json");
    let mocked_data_as_model: Vec<ProductModel> = serde_json::from_str(std::str::from_utf8(mocked_data_as_bytes).unwrap()).unwrap();
    Ok(mocked_data_as_model)
}

pub struct ProductMockedRepository {
    products: Option<Vec<ProductModel>>,
    last_product_created: Option<ProductModel>,
}
impl ProductMockedRepository {
    pub fn new() -> ProductMockedRepository {
        let products = match mocked_products() {
            Ok(value) => Some(value),
            Err(_) => None,
        };
        ProductMockedRepository {
            products,
            last_product_created: None,
        }
    }
}

#[async_trait::async_trait]
impl ProductRepository for ProductMockedRepository {
    fn create(&mut self, product: ProductModel) {
        self.last_product_created = Some(product);
    }

    fn get_all(&self) -> Option<Vec<ProductModel>> {
        self.products.clone()
    }

    async fn save(&mut self, product: ProductModel) {
        todo!()
    }

    async fn get_by_id(&mut self, id: String) -> Option<Vec<ProductModel>> {
        todo!()
    }

    async fn delete_by_id(&mut self, id: String) {
        todo!()
    }
}