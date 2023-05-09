use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::domain::traits::{
    base_traits_entities::Getters,
    repository::{RepositoryError, RepositoryTrait},
};

pub struct MockedRepository<Entity>
where
    Entity: Serialize + Deserialize<'static>,
{
    items: Option<Vec<Entity>>,
    _marker: PhantomData<Entity>,
}
impl<Entity> MockedRepository<Entity>
where
    Entity: Serialize + Deserialize<'static> + 'static + Getters,
{
    pub fn new() -> MockedRepository<Entity> {
        let items;
        let json_data = get_json_bytes::<Entity>().unwrap();
        let json_items: Vec<Entity> = serde_json::from_str(std::str::from_utf8(json_data).unwrap()).unwrap();
        if json_items.is_empty() {
            items = None;
        } else {
            items = Some(json_items);
        }

        MockedRepository { items, _marker: PhantomData }
    }
}

#[async_trait::async_trait]
impl<Entity> RepositoryTrait<Entity> for MockedRepository<Entity>
where
    Entity: Serialize + Deserialize<'static> + Clone + Send + Sync + Getters,
{
    async fn create(&mut self, item: Entity) -> Result<(), RepositoryError> {
        match self.items.as_mut() {
            Some(items) => {
                items.push(item.clone());
            }
            None => {
                self.items = Some(vec![item.clone()]);
            }
        }
        if let Some(it) = self.items.as_ref().unwrap().iter().find(|it| it.get_id() == item.get_id()) {
            Ok(())
        } else {
            Err(RepositoryError::Create(format!("Cannot create item with id: {:?}", item.get_id())))
        }
    }
    async fn update(&mut self, item: Entity) -> Result<(), RepositoryError> {
        if let Some(it) = self.items.as_mut().unwrap().iter_mut().find(|it| it.get_id() == item.get_id()) {
            *it = item;
            Ok(())
        } else {
            Err(RepositoryError::Update(format!(
                "Cannot Find item with id same id: {:?} in repository",
                item.get_id()
            )))
        }
    }
    async fn delete(&mut self, id: String) -> Result<(), RepositoryError> {
        if let Some(it) = self.items.as_mut().unwrap().iter_mut().position(|it| it.get_id().to_string() == id) {
            self.items.as_mut().unwrap().remove(it);
            Ok(())
        } else {
            Err(RepositoryError::DeleteById(format!("Cannot delete item with id: {:?} in repository", id)))
        }
    }
    async fn get_by_id(self, id: String) -> Result<Option<Entity>, RepositoryError> {
        let item;
        if let Some(it) = self.items.as_ref().unwrap().iter().find(|it| it.get_id().to_string() == id) {
            item = it;
            Ok(Some(item.clone()))
        } else {
            return Err(RepositoryError::GetById(format!("Cannot get item by id: {:?} in repository", id)));
        }
    }
    async fn get_all(&self) -> Result<Option<Vec<Entity>>, RepositoryError> {
        Ok(self.items.clone())
    }
}

fn get_json_bytes<Entity>() -> Result<&'static [u8], ()>
where
    Entity: 'static,
{
    // Specify the path to your mocked data
    Ok(include_bytes!("mocked_some_entity.json"))
}
