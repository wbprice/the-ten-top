use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Join, ReadStorage, System, Write, WriteStorage},
};

use crate::{
    components::{Destination, Feeling, Dish, Patron, Register, Task, Velocity},
    resources::{GameState, Tasks, Dishes},
};

pub struct RegisterSystem;

// Put this in a lib!
fn get_distance_between_two_points(point_a: [f32; 2], point_b: [f32; 2]) -> f32 {
    let x = point_a[0] - point_b[0];
    let y = point_a[1] - point_b[1];
    let c: f32 = x.powi(2) + y.powi(2);
    c.sqrt()
}

impl<'s> System<'s> for RegisterSystem {
    type SystemData = (
        Write<'s, GameState>,
        Entities<'s>,
        WriteStorage<'s, Patron>,
        ReadStorage<'s, Register>,
        ReadStorage<'s, Dish>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Destination>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Feeling>,
        WriteStorage<'s, Task>,
    );

    fn run(
        &mut self,
        (
            mut game,
            entities,
            mut patrons,
            registers,
            dishes,
            mut parents,
            mut locals,
            mut destinations,
            mut velocities,
            mut feelings,
            mut tasks,
        ): Self::SystemData,
    ) {
        // For each register
        for (register_entity, register, register_local) in (&entities, &registers, &locals).join() {
            // Check to see if a patron is in range by looking at x value of register...
            let register_translation = register_local.translation();
            let register_x = register_translation.x;
            let register_y = register_translation.y;

            for (patron_entity, patron, patron_local) in (&*entities, &mut patrons, &locals).join()
            {
                // and comparing it to the x of each patron
                let patron_x = patron_local.translation().x;

                if register_x.floor() == patron_x.floor() {
                    // If there's a match, attract the patron
                    // Assign the patron a task (if they don't already have one);
                    if let None = tasks.get(patron_entity) {
                        tasks
                            .insert(
                                patron_entity,
                                Task::new(Tasks::GiveOrder {
                                    register: register_entity,
                                    dish: Dishes::HotDog
                                }),
                            )
                            .unwrap();
                    }
                }
            }
        }
    }
}
