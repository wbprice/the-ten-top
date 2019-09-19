use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{Direction, SimpleAnimation, Velocity, Worker};

pub struct WorkerSystem;

impl<'s> System<'s> for WorkerSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Worker>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SimpleAnimation>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, workers, velocities, mut locals, mut animations, mut sprites, time): Self::SystemData,
    ) {
        for (entity, _, velocity, local) in (&entities, &workers, &velocities, &mut locals).join() {
            local.prepend_translation_x(velocity.x * time.delta_seconds());
            local.prepend_translation_y(velocity.y * time.delta_seconds());

            // If velocity is zeroed out, remove the animation.
            if velocity.x == 0.0 && velocity.y == 0.0 {
                animations.remove(entity);
                let mut sprite = sprites.get_mut(entity).unwrap();
                sprite.sprite_number = 0;
            } else {
                // Fetch the animation so we can decide if we need
                // to set it or not.
                match animations.get(entity) {
                    Some(animation) => {
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
                    None => {
                        animations
                            .insert(entity, SimpleAnimation::new(1, 6, 0.1))
                            .unwrap();
                    }
                }
            }
        }
    }
}
