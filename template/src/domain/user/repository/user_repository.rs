use async_trait::async_trait;

use crate::domain::user::model::user_model::UserModel;

#[async_trait]
pub trait UserRepository {
    fn create(&mut self, user: UserModel);
    fn get_all(&mut self) -> Option<Vec<UserModel>>;
    async fn save(&mut self, user: UserModel);
    async fn get_by_id(&mut self, id: String) -> Option<Vec<UserModel>>;
    async fn delete_by_id(&mut self, id: String);
}
