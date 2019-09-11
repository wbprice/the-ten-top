use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Patron {
    pub satisfaction: u8
}

impl Patron {
    pub fn generate() -> Patron {
        Patron {
            satisfaction: 8
        }
    }
}

impl Component for Patron {
    type Storage = DenseVecStorage<Self>;
}
