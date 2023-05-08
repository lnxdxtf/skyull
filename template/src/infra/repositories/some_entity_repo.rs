use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

pub struct MockedRepository<Entity>
where
    Entity: Serialize + Deserialize<'static>,
{
    items: Vec<Entity>,
    _marker: PhantomData<Entity>,
}
impl<Entity> MockedRepository<Entity>
where
    Entity: Serialize + Deserialize<'static> + 'static,
{
    pub fn new(data: Option<Vec<Entity>>) -> MockedRepository<Entity> {
        let mut items = vec![];
        if data.is_none() {
            let json_data = get_json_bytes::<Entity>().unwrap();
            items = serde_json::from_str(std::str::from_utf8(json_data).unwrap()).unwrap();
        }
        MockedRepository { items, _marker: PhantomData }
    }
}

fn get_json_bytes<Entity>() -> Result<&'static [u8], ()>
where
    Entity: 'static,
{
    // Specify the right path to your mock database
    Ok(include_bytes!("mocked_some_entity.json"))
}
