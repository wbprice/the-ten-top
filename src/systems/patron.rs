use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Direction, Patron, SimpleAnimation, Velocity};

pub struct MovePatronSystem;

impl<'s> System<'s> for MovePatronSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Patron>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SimpleAnimation>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, patrons, velocities, mut locals, mut animations, mut sprites, time): Self::SystemData,
    ) {
        for (entity, _, velocity, local) in (&entities, &patrons, &velocities, &mut locals).join() {
            local.prepend_translation_x(velocity.x * time.delta_seconds());
            local.prepend_translation_y(velocity.y * time.delta_seconds());

            // Allow patrons to run off the screen
            // If they do so, update their position to the opposite edge of the screen.
            let patron_x = local.translation().x;
            if patron_x > 200.0 {
                local.set_translation_x(0.0);
            }

            if patron_x < -40.0 {
                local.set_translation_x(160.0);
            }

            // If velocity is zeroed out, remove the animation.
            if velocity.x == 0.0 && velocity.y == 0.0 {
                animations.remove(entity);
                let mut sprite = sprites.get_mut(entity).unwrap();
                sprite.sprite_number = 6;
            } else {
                // Fetch the animation so we can decide if we need
                // to set it or not.
                match animations.get(entity) {
                    Some(animation) => match velocity.get_direction() {
                        _ => {
                            if animation.start_sprite_index != 6 {
                                animations
                                    .insert(entity, SimpleAnimation::new(6, 6, 0.1))
                                    .unwrap();
                            }
                        }
                    },
                    None => {
                        animations
                            .insert(entity, SimpleAnimation::new(6, 6, 0.1))
                            .unwrap();
                    }
                }
            }
        }
    }
}
