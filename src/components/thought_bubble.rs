use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ThoughtBubble {
    pub offset: [f32; 2],
}

impl ThoughtBubble {
    pub fn new() -> ThoughtBubble {
        ThoughtBubble {
            offset: [0.0, 12.0],
        }
    }
}

impl Component for ThoughtBubble {
    type Storage = DenseVecStorage<Self>;
}
