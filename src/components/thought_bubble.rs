use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct ThoughtBubble;

impl Component for ThoughtBubble {
    type Storage = DenseVecStorage<Self>;
}
