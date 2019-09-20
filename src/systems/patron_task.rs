use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadStorage, System, Write, WriteStorage},
};

use crate::{
    components::{Destination, Food, Subtask, Task, Worker},
    resources::{GameState, Status, Subtasks, Tasks},
};

pub struct PatronTaskSystem;

impl<'s> System<'s> for PatronTaskSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Patron>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Food>,
        WriteStorage<'s, Destination>,
        WriteStorage<'s, Task>,
        WriteStorage<'s, Parent>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut patrons,
            mut locals,
            foods,
            mut destinations,
            mut tasks,
            mut parents
        )
    ) {
        let mut tasks_to_remove : Vec<Entity> = vec![];

        // Perform first time setup when the patron is handed a task.
        for (patron_entity, patron, task) in (&entities, &patrons, &tasks)
            .join()
            .filter(|(_, _, task)| task.status == Status::New) {
            
            // Populate task subactivities based on type of activity.
            match task.activity {
                Tasks::MakeOrder { register, dish } => {
                    task.subtasks.push(Subtask::new(
                        Subtasks::MoveToEntity {
                            entity: register
                        }
                    });
                    task.subtasks.push(Subtask::new(
                        Subtasks::WaitForWorker
                    ));
                    task.subtasks.push(SubTask::new(
                        Subtasks::SubmitOrder { dish }
                    ));
                    task.subtasks.push(SubTask::new(
                        Subtasks::WaitForOrder { dish }
                    ));
                    task.subtasks.push(SubTask::new(
                        Subtasks::MoveTo { destination: Destination {
                            x: 144.0,
                            y: 30.0
                        }}
                    ));
                }
            }
        }
    }
}
