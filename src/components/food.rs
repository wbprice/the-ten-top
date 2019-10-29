use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Copy, Clone)]
pub struct Food {
    pub food: Foods,
}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}
