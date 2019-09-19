use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    components::{Task, Subtask, Destination, Food, Dish, Patron, Velocity, Worker},
    resources::{GameState, Status, Tasks, Subtasks},
};

pub struct TaskSystem;

impl<'s> System<'s> for TaskSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Worker>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Food>,
        WriteStorage<'s, Destination>,
        WriteStorage<'s, Task>,
        Write<'s, GameState>,
    );

    fn run(
        &mut self,
        (
            entities,
            workers,
            locals,
            foods,
            mut destinations,
            mut tasks,
            mut game_state,
        ): Self::SystemData,
    ) {
        let mut tasks_to_remove: Vec<Entity> = vec![];

        // If a new task needs to be performed
        // Find an idle worker and assign them the task.
        if let Some(task) = game_state.tasks.pop() {
            match (&entities, &workers, !&tasks).join().next() {
                Some((worker_entity, _, _)) => {
                    // Note that worker is busy with this task
                    let mut task = Task::new(task);

                    // populate subtasks based on task?
                    match task.activity {
                        Tasks::TakeOrder { patron } => {
                            task.subtasks.push(Subtask {
                                activity: Subtasks::MoveToEntity { entity: patron },
                                status: Status::New
                            });
                        },
                        Tasks::DeliverOrder { patron, dish } => {
                            match (&entities, &foods).join().find(|(_, food)| food.dish == dish) {
                                Some((food_entity, _)) => {
                                    task.subtasks.push(Subtask {
                                        activity: Subtasks::MoveToEntity { entity: food_entity },
                                        status: Status::New
                                    });
                                    task.subtasks.push(Subtask {
                                        activity: Subtasks::PickUpEntity { entity: food_entity },
                                        status: Status::New
                                    });
                                    task.subtasks.push(Subtask {
                                        activity: Subtasks::MoveToEntity { entity: patron },
                                        status: Status::New
                                    });
                                    task.subtasks.push(Subtask {
                                        activity: Subtasks::SetEntityOwner { entity: food_entity, owner: patron },
                                        status: Status::New
                                    });
                                },
                                None => {
                                    dbg!("That kind of food hasn't been made yet!");
                                }
                            }
                        },
                        _ => {
                            unimplemented!();
                        }
                    }

                    // Add the new task to the tasks storage
                    tasks
                        .insert(worker_entity, task)
                        .unwrap();
                }
                None => {
                    // no idle workers!
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
            match task.subtasks.iter_mut().find(|subtask| subtask.status != Status::Completed) {
                Some(mut subtask) => {
                    match subtask.activity {
                        Subtasks::MoveToEntity { entity } => {
                            match subtask.status {
                                Status::New => {
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
                                    // Destination storage will remove the destination
                                    // once the entity has reached it's destination.
                                    // So the destination no longer exists, we can call
                                    // the task done.
                                    if let None = destinations.get(worker_entity) {
                                        subtask.status = Status::Completed
                                        // Perform any cleanup
                                    }
                                },
                                Status::Completed => {
                                    unreachable!();
                                }
                                Status::Blocked => {
                                    unimplemented!();
                                },
                            }
                        }
                        _ => {}

                    }
                },
                None => {
                    // Perform any cleanup and remove the assignment
                    dbg!("the task is completed");
                    tasks_to_remove.push(worker_entity);
                }
            }
        }

        // Cleanup any completed assignments, marking the worker available for the next task.
        for worker_entity in tasks_to_remove {
            match tasks.remove(worker_entity) {
                Some(thing) => {
                    dbg!(thing);
                    dbg!("removed task");
                }
                None => {
                    dbg!("nothing to remove?");
                }
            }
        }
    }
}
