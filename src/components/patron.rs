use amethyst::ecs::prelude::{
    Component,
    DenseVecStorage
};

enum Food {
    HAMBURGER,
    HOT_DOG,
    TACO,
    ELOTE
    TAKOYAKI,
    FISHBALLS,
    BANH_MI,
    PHO
    // and others, I guess
}

struct Patron {
    satisfaction: u8,
    velocity: [f32; 2],
    craving: Food
}

impl Component for Patron {
    type Storage = DenseVecStorage<Self>;
}