use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    components::{Cupboard, Ingredients, Ingredient},
    common::{Point}
};

pub fn init_ingredient(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, point: Point, ingredient : Ingredients) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(point.x, point.y, 0.2);

    world.create_entity()
        .with(Ingredient {
            ingredient
        })
        .with(local_transform)
        .build();
}