use async_trait::async_trait;

use crate::domain::product::model::product_model::ProductModel;

#[async_trait]
pub trait ProductRepository {
    fn create(&mut self, product: ProductModel);
    fn get_all(&mut self) -> Option<Vec<ProductModel>>;
    async fn save(&mut self, product: ProductModel);
    async fn get_by_id(&mut self, id: String) -> Option<Vec<ProductModel>>;
    async fn delete_by_id(&mut self, id: String);
}
    