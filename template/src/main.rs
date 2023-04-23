use crate::adapters::database::memory::product_repository::memory_product_repository::ProductDatabaseJsonRepository;
use domain::product::repository::product_repository::ProductRepository;

mod adapters;
mod application;
mod domain;

fn main() {
    let mut productRepository = ProductDatabaseJsonRepository::new();
    let a = productRepository.get_all();
    println!("{:?}", a);
}
