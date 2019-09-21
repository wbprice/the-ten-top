use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadStorage, System, Write, WriteStorage},
};

use crate::{
    components::{Destination, Food, Subtask, Task, Worker},
    resources::{GameState, Status, Subtasks, Tasks},
};

pub struct WorkerTaskSystem;

impl<'s> System<'s> for WorkerTaskSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Worker>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Food>,
        WriteStorage<'s, Destination>,
        WriteStorage<'s, Task>,
        WriteStorage<'s, Parent>,
        Write<'s, GameState>,
    );

    fn run(
        &mut self,
        (
            entities,
            workers,
            mut locals,
            foods,
            mut destinations,
            mut tasks,
            mut parents,
            mut game_state,
        ): Self::SystemData,
    ) {
        let mut tasks_to_remove: Vec<Entity> = vec![];

        // If a new task needs to be performed
        // Find an idle worker and assign them the task.
        if let Some(task) = game_state.tasks.first() {
            match (&entities, &workers, !&tasks).join().next() {
                Some((worker_entity, _, _)) => {
                    // Note that worker is busy with this task
                    let mut task = Task::new(*task);

                    dbg!("New task to assign!");
                    dbg!(&task);

                    // populate subtasks based on task
                    match task.activity {
                        Tasks::TakeOrder { patron } => {
                            task.subtasks.push(Subtask {
                                activity: Subtasks::MoveToEntity { entity: patron },
                                status: Status::New,
                            });
                        }
                        Tasks::DeliverOrder { patron, dish } => {
                            match (&entities, &foods)
                                .join()
                                .find(|(_, food)| food.dish == dish)
                            {
                                Some((food_entity, _)) => {
                                    task.subtasks.push(Subtask {
                                        activity: Subtasks::MoveToEntity {
                                            entity: food_entity,
                                        },
                                        status: Status::New,
                                    });
                                    task.subtasks.push(Subtask {
                                        activity: Subtasks::SetEntityOwner {
                                            entity: food_entity,
                                            owner: worker_entity,
                                        },
                                        status: Status::New,
                                    });
                                    task.subtasks.push(Subtask {
                                        activity: Subtasks::MoveToEntity { entity: patron },
                                        status: Status::New,
                                    });
                                    task.subtasks.push(Subtask {
                                        activity: Subtasks::SetEntityOwner {
                                            entity: food_entity,
                                            owner: patron,
                                        },
                                        status: Status::New,
                                    });
                                    task.subtasks.push(Subtask {
                                        activity: Subtasks::MoveTo {
                                            destination: Destination { x: 44.0, y: 108.0 },
                                        },
                                        status: Status::New,
                                    });
                                }
                                None => {
                                    unimplemented!("That kind of food hasn't been made yet!");
                                }
                            }
                        }
                        _ => {
                            unimplemented!("That task hasn't been implemented yet!");
                        }
                    }

                    // Remove the task from the queue.
                    game_state.tasks.pop().unwrap();
                    // Add the new task to the tasks storage
                    tasks.insert(worker_entity, task).unwrap();
                }
                None => {
                    // no idle workers!
                    dbg!("No idle workers available to take on this task");
                }
            }
        }

        // For workers with subtasks to complete:
        // Perform initial setup for each subtask
        // Check to see if subtask was completed
        // Move onto the next subtask until there
        // aren't any more subtasks
        for (worker_entity, _, task) in (&entities, &workers, &mut tasks).join() {
            // Find the next uncompleted task
            match task
                .subtasks
                .iter_mut()
                .find(|subtask| subtask.status != Status::Completed)
            {
                Some(mut subtask) => {
                    match subtask.activity {
                        Subtasks::MoveToEntity { entity } => {
                            match subtask.status {
                                Status::New => {
                                    dbg!("[MoveToEntity] worker task started");
                                    // Where is the entity to walk to?
                                    let entity_local = locals.get(entity).unwrap();
                                    let entity_transform = entity_local.translation();

                                    // Direct the worker to walk to the entity
                                    destinations
                                        .insert(
                                            worker_entity,
                                            Destination {
                                                x: entity_transform.x,
                                                y: entity_transform.y,
                                            },
                                        )
                                        .unwrap();

                                    subtask.status = Status::InProgress;
                                }
                                Status::InProgress => {
                                    dbg!("[MoveToEntity] worker walking to destination");
                                    // Destination storage will remove the destination
                                    // once the entity has reached it's destination.
                                    // So the destination no longer exists, we can call
                                    // the task done.
                                    if let None = destinations.get(worker_entity) {
                                        subtask.status = Status::Completed
                                        // Perform any cleanup
                                    }
                                }
                                Status::Completed => {
                                    unreachable!();
                                }
                                Status::Blocked => {
                                    unimplemented!("[MoveToEntity] blocked tasks haven't been implemented yet!");
                                }
                            }
                        }
                        Subtasks::SetEntityOwner { entity, owner } => {
                            dbg!("[SetEntityOwner] start set entity owner task");
                            // Attach the entity to the new owner.
                            parents.insert(entity, Parent { entity: owner }).unwrap();
                            // Reset the entity local
                            let mut local = Transform::default();
                            local.prepend_translation_z(0.2);
                            local.prepend_translation_x(8.0);
                            local.prepend_translation_y(-8.0);
                            // Insert the local to the locals storage.
                            locals.insert(entity, local).unwrap();
                            subtask.status = Status::Completed;
                            dbg!("[SetEntityOwner] start set entity owner completed");
                        }
                        Subtasks::MoveTo { destination } => {
                            match subtask.status {
                                Status::New => {
                                    destinations.insert(worker_entity, destination).unwrap();
                                    subtask.status = Status::InProgress;
                                }
                                Status::InProgress => {
                                    if let None = destinations.get(worker_entity) {
                                        subtask.status = Status::Completed
                                        // Perform any cleanup
                                    }
                                }
                                Status::Completed => {
                                    unreachable!();
                                }
                                Status::Blocked => {
                                    unimplemented!();
                                }
                            }
                        }
                        _ => {}
                    }
                }
                None => {
                    // Perform any cleanup and remove the assignment
                    tasks_to_remove.push(worker_entity);
                }
            }
        }

        // Cleanup any completed assignments, marking the worker available for the next task.
        for worker_entity in tasks_to_remove {
            tasks.remove(worker_entity).unwrap();
            dbg!("Worker free for reassignment");
        }
    }
}
