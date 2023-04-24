use super::product::repository::product_repository::ProductRepository;

pub trait RepositoriesTrait {
    fn product(&self) -> &dyn ProductRepository;
    // fn user(&self) -> &dyn UserRepository;
}
