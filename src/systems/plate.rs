use amethyst::{
    core::timing::Time,
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadStorage, Read, System, WriteStorage},
    renderer::SpriteRender
};

use crate::{
    components::{
        Plate,
        Ingredient
    }
};

pub struct PlateSystem;

impl<'s> System<'s> for PlateSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Plate>,
        WriteStorage<'s, Ingredient>
    );

    fn run(&mut self, (entities, parents, plates, ingredients): Self::SystemData) {
        for (plate_entity, plate) in (&entities, &plates).join() {
            // Iterate through the plates.
            // What ingredients have this plate as a parent?
            let ingredients : Vec<(Entity, Parent, Ingredient)> = (&entities, &parents, &ingredients)
                .join()
                .filter(|(_, parent, _)| parent.entity == plate_entity)
                .collect();

            dbg!("ingredients on the plate");
            dbg!(&ingredients);

        }
    }
}