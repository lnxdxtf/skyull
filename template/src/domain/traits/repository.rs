use std::io;
use std::marker::PhantomData;
use thiserror::Error;

// Use this every time you need create a new instance of a repository for any entity type
// Where Repository is a repository generic for a entity
// ex: Repository<Entity>, but the Repository<Entity> is just a repository generic like Repository
pub struct RepositoryGeneric<Repository> {
    pub repository: Repository,
    _marker: PhantomData<Repository>,
}
impl<Repository> RepositoryGeneric<Repository> {
    pub fn new(repository: Repository) -> RepositoryGeneric<Repository> {
        RepositoryGeneric {
            repository,
            _marker: PhantomData,
        }
    }
}

// How to use:
// let repo = Repository::new(Repository_Entity).repository;
// where Repository_Entity: struct that implements trait RepositoryTrait<Entity>

#[async_trait::async_trait]
pub trait RepositoryTrait<Entity> {
    async fn create(&mut self, item: Entity) -> Result<(), RepositoryError>;
    async fn update(&mut self, item: Entity) -> Result<(), RepositoryError>;
    async fn delete(&mut self, id: String) -> Result<(), RepositoryError>;
    async fn get_by_id(self, id: String) -> Result<Option<Entity>, RepositoryError>;
    async fn get_all(&self) -> Result<Option<Vec<Entity>>, RepositoryError>;
}

// If you want to use this default enum errors... go ahead
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Cannot connect the repository to database")]
    Disconect(#[from] io::Error),

    #[error("Cannot create")]
    Create(String),

    #[error("Cannot update")]
    Update(String),

    #[error("Cannot Get by id")]
    GetById(String),

    #[error("Cannot Get All")]
    GetALL(String),

    #[error("Cannot Delete by id")]
    DeleteById(String),
}
