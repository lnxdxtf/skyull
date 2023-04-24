use adapters::database::repositories::Repositories;
use domain::repositories_trait::RepositoriesTrait;
use dotenv::dotenv;

extern crate dotenv;

mod adapters;
mod application;
mod domain;

fn main() {
    dotenv().ok();
    let rep = Repositories::new();
    let b = rep.product().get_all();
    println!("{:?}", b);
}
