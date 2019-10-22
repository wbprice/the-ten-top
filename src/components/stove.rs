use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Stove {
    pub cook_time: f32,
}

impl Stove {
    pub fn new() -> Stove {
        Stove { cook_time: 0.0 }
    }
}

impl Component for Stove {
    type Storage = DenseVecStorage<Self>;
}
