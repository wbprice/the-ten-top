use crate::resources::Status;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Patron {
    pub satisfaction: u8,
    pub order_status: Status,
}

impl Patron {
    pub fn generate() -> Patron {
        Patron {
            satisfaction: 8,
            order_status: Status::New,
        }
    }
}

impl Component for Patron {
    type Storage = DenseVecStorage<Self>;
}
