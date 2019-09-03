use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

pub fn init_thought_bubble(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(12.0, 24.0, 0.);
    
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Patron {
            satisfaction: 100,
            velocity: [25.0, 0.0],
            craving: Food::Hamburger,
        })
        .with(SimpleAnimation::new(0, 6, 0.1))
        .with(local_transform)
        .build();
}