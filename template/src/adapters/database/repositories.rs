use super::mocked::product_repository::mocked_repository_impl::ProductMockedRepository;
use crate::domain::{
    product::repository::product_repository::ProductRepository, repositories_trait::RepositoriesTrait,
    user::repository::user_repository::UserRepository,
};
use std::{env, str::FromStr};

pub struct Repositories {
    product_repository: Box<dyn ProductRepository>,
    // user_repository: Box<dyn UserRepository>,
}

impl RepositoriesTrait for Repositories {
    fn product(&self) -> &dyn ProductRepository {
        &*self.product_repository
    }
    // fn user(&self) -> &dyn UserRepository {
    //     &*self.user_repository
    // }
}

impl Repositories {
    pub fn new() -> Repositories {
        let database_environment = env::var("DATABASE_SET").unwrap();
        let database = database_environment.parse::<DatabaseSet>().unwrap();

        let product_repository;
        // let user_repository;
        match database {
            DatabaseSet::Mocked => {
                product_repository = Box::new(ProductMockedRepository::new());
                // user_repository =  Box::new(UserDatabaseJsonRepository);
            }
            DatabaseSet::MongoDB => todo!(),
            DatabaseSet::Redis => todo!(),
        }

        Repositories { product_repository }
    }
}

pub enum DatabaseSet {
    Mocked, // Used for testing
    MongoDB,
    Redis,
}
impl FromStr for DatabaseSet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Mocked" => Ok(DatabaseSet::Mocked),
            "MongoDB" => Ok(DatabaseSet::MongoDB),
            "Redis" => Ok(DatabaseSet::Redis),
            _ => Err(()),
        }
    }
}
