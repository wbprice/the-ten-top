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
                Tasks::DeliverOrder { patron, food } => {
                    match task.status {
                        Status::New => {
                            // In order for food to be deliverable, it has to exist.
                            // If the food exists, tell a worker to pick it up and take it to the patron
                            // If the food doesn't exist, mark this task blocked
                            // Put a task "PrepOrder" into the backlog for creating the food
                            match (&entities, &foods).join().find(|(_, f)| f.food == food) {
                                Some(food_entity) => {
                                    task.status = Status::Actionable;
                                },
                                None => { 
                                    task.status = Status::Blocked;
                                    game_state.tasks.push(Task::new(Tasks::PrepOrder { food }));
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
                                                task.status = Status::Actionable;
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
                _ => {
                    dbg!("task not implemented yet! Mark it done!");
                    task.status = Status::Completed;
                }
            }
        }
    }
}
