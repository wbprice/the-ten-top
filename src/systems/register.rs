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

    fn run(&mut self, (patrons, registers, locals, time): Self::SystemData) {
        // For each register
        for (register, register_local) in (&registers, &locals).join() {
            // Check to see if a patron is in range
            for (patron, patron_local) in (&patrons, &locals).join() {
                dbg!(patron);
            }
        }
    }
}