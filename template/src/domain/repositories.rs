use super::{product::repository::product_repository::ProductRepository, user::repository::user_repository::UserRepository};

pub trait Repositories {
    fn product(&self) -> &dyn ProductRepository;
    // fn user(&self) -> &dyn UserRepository;
}
