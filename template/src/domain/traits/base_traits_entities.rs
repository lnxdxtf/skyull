use uuid::Uuid;

pub trait Getters {
    fn get_id(&self) -> Uuid;
}

pub trait Setters {
    fn set_name(&mut self);
}
