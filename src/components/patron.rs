use amethyst::ecs::prelude::{Component, DenseVecStorage};
use rand::Rng;

#[derive(Debug)]
pub struct Patron {
    pub satisfaction: u8,
    pub velocity: [f32; 2],
    pub origin: [f32; 2]
}

impl Patron {
    pub fn generate() -> Patron {
        let mut rng = rand::thread_rng();

        Patron {
            satisfaction: rng.gen_range(0, 8),
            velocity: [
                rng.gen_range(15.0, 25.0),
                0.0
            ],
            origin: [
                -10.0,
                rng.gen_range(0.0, 48.0)
            ]
        }
    }
}

impl Component for Patron {
    type Storage = DenseVecStorage<Self>;
}
