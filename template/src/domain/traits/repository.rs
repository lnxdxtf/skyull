use std::io;
use std::marker::PhantomData;
use thiserror::Error;

// This struct stores all repositories in a single place.
// Use this to centralize all repositories.
// If you need more repositories, add a generic parameter and a corresponding field in the struct, and implement it in the `new` function.
pub struct RepositoriesApp<RepositoryX, RepositoryY> {
    pub repository_x: RepositoryX,
    pub repository_y: RepositoryY,
    _marker: PhantomData<(RepositoryX, RepositoryY)>,
}

impl<RepositoryX, RepositoryY> RepositoriesApp<RepositoryX, RepositoryY> {
    // Creates a new instance of RepositoriesApp with the provided repositories.
    pub fn new(repository_x: RepositoryX, repository_y: RepositoryY) -> RepositoriesApp<RepositoryX, RepositoryY> {
        RepositoriesApp {
            repository_x,
            repository_y,
            _marker: PhantomData,
        }
    }
}

// This is the default-generic implementation for any repository structure (e.g., database, mocked, local, etc.).
// It needs to be implemented for each repository type.
#[async_trait::async_trait]
pub trait RepositoryTrait<Entity> {
    async fn create(&mut self, item: Entity) -> Result<(), RepositoryError>;
    async fn update(&mut self, item: Entity) -> Result<(), RepositoryError>;
    async fn delete(&mut self, id: String) -> Result<(), RepositoryError>;
    async fn get_by_id(self, id: String) -> Result<Option<Entity>, RepositoryError>;
    async fn get_all(&self) -> Result<Option<Vec<Entity>>, RepositoryError>;
}

// Default implementation for errors in repository operations.
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Failed to connect the repository to the database")]
    Disconnected(#[from] io::Error),

    #[error("Failed to create")]
    Create(String),

    #[error("Failed to update")]
    Update(String),

    #[error("Failed to retrieve by ID")]
    GetById(String),

    #[error("Failed to retrieve all")]
    GetAll(String),

    #[error("Failed to delete by ID")]
    DeleteById(String),
}
