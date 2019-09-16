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
        for (entity, _, velocity, local) in
            (&entities, &patrons, &velocities, &mut locals).join()
        {
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
                sprite.sprite_number = 0;
            } else {
                // Fetch the animation so we can decide if we need
                // to set it or not.
                let animation = animations.get(entity).unwrap();
                match velocity.get_direction() {
                    Direction::Up => {
                        // TODO: Update with walking up sprites
                        if animation.start_sprite_index != 1 {
                            println!("Change to walking up");
                            animations
                                .insert(entity, SimpleAnimation::new(1, 6, 0.1))
                                .unwrap();
                        }
                    }
                    Direction::Down => {
                        // TODO: Update with walking down sprites
                        if animation.start_sprite_index != 1 {
                            println!("Change to walking down");
                            animations
                                .insert(entity, SimpleAnimation::new(1, 6, 0.1))
                                .unwrap();
                        }
                    }
                    Direction::Left => {
                        // TODO: Update with walking left sprites
                        if animation.start_sprite_index != 1 {
                            println!("Change to walking left");
                            animations
                                .insert(entity, SimpleAnimation::new(1, 6, 0.1))
                                .unwrap();
                        }
                    }
                    _ => {
                        if animation.start_sprite_index != 1 {
                            println!("Change to walking right");
                            animations
                                .insert(entity, SimpleAnimation::new(1, 6, 0.1))
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
}
