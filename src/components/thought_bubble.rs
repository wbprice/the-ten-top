use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ThoughtBubble;

impl Component for ThoughtBubble {
    type Storage = DenseVecStorage<Self>;
}
