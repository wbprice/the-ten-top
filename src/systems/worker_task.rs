use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadStorage, System, Write, WriteStorage},
};

use crate::{
    components::{Destination, Food, Foods, Ingredient, Ingredients, Subtask, Task, Worker},
    resources::{GameState, Status, Subtasks, Tasks},
};

pub struct WorkerTaskSystem;

impl<'s> System<'s> for WorkerTaskSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Worker>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Food>,
        ReadStorage<'s, Ingredient>,
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
            ingredients,
            mut destinations,
            mut tasks,
            mut parents,
            mut game_state,
        ): Self::SystemData,
    ) {
        
        // If a new task was added to the backlog, figure out if it is actionable.
        // If so, assign it to a worker.
        // If not, mark it "blocked" and find out why it's not actionable yet.
        // Find out what actions need to occur next and put those in the backlog

        let mut tasks_to_add_to_backlog : Vec<Task> = vec![];
        let mut tasks_to_assign : Vec<(Entity, usize)> = vec![];

        for task in game_state.tasks.iter_mut() {
            match task.activity {
                // Taking an order consists of walking to the patron and asking what they
                // want to order.
                Tasks::TakeOrder { patron } => {
                    match task.status {
                        Status::New => {
                            // This task doesn't have any prerequisites,
                            // So status can be set to "Actionable" immediately.
                            task.status = Status::Actionable;
                            task.subtasks.push(Subtask::new(Subtasks::MoveToEntity { entity: patron }));
                        },
                        Status::Blocked => {
                            unreachable!("Take order can't be blocked");
                        },
                        _ => {}
                    }
                },
                Tasks::DeliverOrder { patron: _, food } => {
                    match task.status {
                        Status::New => {
                            // In order for food to be deliverable, it has to exist.
                            // If the food exists, tell a worker to pick it up and take it to the patron
                            // If the food doesn't exist, mark this task blocked
                            // Put a task "PrepOrder" into the backlog for creating the food
                            match (&entities, &foods).join().find(|(_, f)| f.food == food) {
                                Some(_) => {
                                    task.status = Status::Actionable;
                                },
                                None => { 
                                    task.status = Status::Blocked;
                                    tasks_to_add_to_backlog.push(Task::new(Tasks::PrepOrder { food }));
                                }
                            }
                        },
                        Status::Blocked => {
                            // This task can set to actionable if a food of the given type is found.
                            if let Some(_) = (&foods).join().find(|f| f.food == food) {
                                task.status = Status::Actionable
                            }
                        }
                        _ => {}
                    }
                },
                Tasks::PrepOrder { food } => {
                    match task.status {
                        Status::New => {
                            // In order to prep an order, all the ingredients need to exist.
                            // If the ingredients exist, tell a worker to pick them up and put them on a plate.
                            // If the ingredients don't exist, well, that's a problem :)
                            match food {
                                Foods::HotDog => {
                                    let hot_dog_ingredients : Vec<Ingredients> = vec![
                                        Ingredients::HotDogWeiner,
                                        Ingredients::HotDogBun
                                    ];

                                    match hot_dog_ingredients
                                        .iter()
                                        .all(|target| {
                                            if let Some(_) = (&ingredients).join().find(|ingred| ingred.ingredient == *target) { 
                                                return true
                                            } 
                                            return false
                                        }) {
                                            true => {
                                                task.status = Status::Actionable;
                                            },
                                            false => {
                                                task.status = Status::Blocked;
                                            }
                                        }
                                }
                                _ => {
                                    unimplemented!();
                                }
                            }
                        },
                        _ => {}
                    }
                },
                _ => {
                    unimplemented!();
                }
            }
        }

        // Put any new tasks into the backlog to be addressed on the next tick.
        for task in tasks_to_add_to_backlog {
            game_state.tasks.push(task);
        }

        // If there are any actionable tasks, find an available worker to take it on.
        // Depending on the task, a number of subtasks can be scheduled.
        // These are usually specific to the worker.
        if let Some((index, task)) = game_state.tasks.iter_mut().enumerate().find(|(_, t)| t.status == Status::Actionable) {
            // Find an available worker to take on the task.

            dbg!("A new task is ready!");
            if let Some((worker_entity, _, _)) = (&entities, &workers, !&tasks).join().next() {
                dbg!("A new worker is ready to take it on!");
                match task.activity {
                    Tasks::TakeOrder { patron } => {
                        task.subtasks.push(Subtask::new(Subtasks::MoveToEntity { entity: patron }));
                        task.status = Status::InProgress;
                    },
                    Tasks::DeliverOrder { patron, food } => {
                        match (&entities, &foods).join().find(|(_, f)| f.food == food) {
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
                    },
                    _ => {
                        unimplemented!("That task hasn't been implemented yet!");
                    }
                }

                // Mark the task "InProgress"
                // Assign it to the worker.
                task.status = Status::InProgress;
                tasks_to_assign.push((worker_entity, index));
            }
        }

        for (worker_entity, index) in tasks_to_assign {
            let task_to_assign = &game_state.tasks[index];
            tasks.insert(worker_entity, *task_to_assign).unwrap();
        }
    }
}
