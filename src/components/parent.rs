use amethyst::ecs::prelude::{Component, Entity, DenseVecStorage};

#[derive(Debug)]
pub struct Parent {
    pub entity: Entity
}

impl Component for Parent {
    type Storage = DenseVecStorage<Self>;
}
