use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{
    Patron,
    Register
};

pub struct RegisterSystem;

impl<'s> System<'s> for RegisterSystem {
    type SystemData = (
        WriteStorage<'s, Patron>,
        ReadStorage<'s, Register>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut patrons, registers, locals, time): Self::SystemData) {
        // For each register
        for (register, register_local) in (&registers, &locals).join() {
            // Check to see if a patron is in range by looking at x value of register...
            let register_x = register_local.translation().x;

            for (patron, patron_local) in (&mut patrons, &locals).join() {
                // and comparing it to the x of each patron
                let patron_x = patron_local.translation().x;

                // If there's a match, attract the patron
                if register_x.floor() == patron_x.floor() {
                    dbg!("should attract patron");
                    dbg!(patron);
                }
            }
        }
    }
}