use amethyst::ecs::prelude::{
    Component,
    DenseVecStorage
};

use crate::components::Food;

pub struct Patron {
    satisfaction: u8,
    velocity: [f32; 2],
    craving: Food
}

impl Component for Patron {
    type Storage = DenseVecStorage<Self>;
}