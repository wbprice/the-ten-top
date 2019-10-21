use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadStorage, System, Write, WriteStorage},
};

use crate::{
    components::{Destination, Food, Foods, Ingredient, Ingredients, Plate, Subtask, Task, Worker},
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
        ReadStorage<'s, Plate>,
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
            plates,
            mut parents,
            mut game_state,
        ): Self::SystemData,
    ) {
        // If a new task was added to the backlog, figure out if it is actionable.
        // If so, assign it to a worker.
        // If not, mark it "blocked" and find out why it's not actionable yet.
        // Find out what actions need to occur next and put those in the backlog

        let mut tasks_to_add_to_backlog: Vec<Task> = vec![];
        let mut tasks_to_assign: Vec<(Entity, Task)> = vec![];
        let mut tasks_to_unassign: Vec<Entity> = vec![];

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
                            task.subtasks
                                .push(Subtask::new(Subtasks::MoveToEntity { entity: patron }));
                        }
                        Status::Blocked => {
                            unreachable!("Take order can't be blocked");
                        }
                        _ => {}
                    }
                }
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
                                }
                                None => {
                                    task.status = Status::Blocked;
                                    tasks_to_add_to_backlog
                                        .push(Task::new(Tasks::PrepOrder { food }));
                                }
                            }
                        }
                        Status::Blocked => {
                            // This task can set to actionable if a food of the given type is found.
                            if let Some(_) = (&foods).join().find(|f| f.food == food) {
                                task.status = Status::Actionable
                            }
                        }
                        _ => {}
                    }
                }
                Tasks::PlateIngredient { plate, ingredient } => {
                    match task.status {
                        Status::New => {
                            // If the ingredient exists, it can be plated.
                            // If it doesn't exist, mark the task blocked.
                            match (&ingredients)
                                .join()
                                .find(|ingred| ingred.ingredient == ingredient)
                            {
                                Some(_) => task.status = Status::Actionable,
                                None => task.status = Status::Blocked,
                            }
                        }
                        Status::Blocked => {
                            // If the ingredient exists, it can be plated.
                            // Mark the task actionable
                            if let Some(_) = (&ingredients)
                                .join()
                                .find(|ingred| ingred.ingredient == ingredient)
                            {
                                task.status = Status::Actionable;
                            }
                        }
                        _ => {}
                    }
                }
                Tasks::PrepOrder { food } => {
                    match task.status {
                        Status::New => {
                            // In order to prep an order, all the ingredients need to exist.
                            // If the ingredients exist, tell a worker to pick them up and put them on a plate.
                            // If the ingredients don't exist, well, that's a problem :)

                            // Find an empty plate
                            match (&entities, &plates).join().next() {
                                Some((plate_entity, plate)) => {
                                    match food {
                                        Foods::HotDog => {
                                            // TODO, maybe this lives on a map somewhere?
                                            let hot_dog_ingredients: Vec<Ingredients> = vec![
                                                Ingredients::HotDogWeiner,
                                                Ingredients::HotDogBun,
                                            ];

                                            for ingredient in hot_dog_ingredients {
                                                tasks_to_add_to_backlog.push(Task::new(
                                                    Tasks::PlateIngredient {
                                                        ingredient,
                                                        plate: plate_entity,
                                                    },
                                                ));
                                            }
                                        }
                                        _ => {
                                            unimplemented!();
                                        }
                                    }
                                }
                                None => {
                                    // If no empty plates, that's a blocker
                                    task.status = Status::Blocked;
                                }
                            }
                        }
                        _ => {}
                    }
                }
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
        if let Some(task) = game_state
            .tasks
            .iter_mut()
            .find(|t| t.status == Status::Actionable)
        {
            // Find an available worker to take on the task.
            dbg!("A new task is ready!");
            if let Some((worker_entity, _, _)) = (&entities, &workers, !&tasks).join().next() {
                dbg!("A new worker is ready to take it on!");
                match task.activity {
                    Tasks::TakeOrder { patron } => {
                        task.subtasks
                            .push(Subtask::new(Subtasks::MoveToEntity { entity: patron }));
                        task.status = Status::InProgress;
                    }
                    Tasks::PlateIngredient { ingredient, plate } => {
                        if let Some((ingredient_entity, ingredient)) = (&entities, &ingredients)
                            .join()
                            .find(|(e, i)| i.ingredient == ingredient)
                        {
                            task.subtasks.push(Subtask::new(Subtasks::MoveToEntity {
                                entity: ingredient_entity,
                            }));
                            task.subtasks.push(Subtask::new(Subtasks::SetEntityOwner {
                                entity: ingredient_entity,
                                owner: worker_entity,
                            }));
                            task.subtasks
                                .push(Subtask::new(Subtasks::MoveToEntity { entity: plate }));
                            task.subtasks.push(Subtask::new(Subtasks::SetEntityOwner {
                                entity: ingredient_entity,
                                owner: plate,
                            }));
                        }
                    }
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
                    }
                    _ => {
                        unimplemented!("That task hasn't been implemented yet!");
                    }
                }

                // Mark the task "InProgress"
                // Assign it to the worker.
                task.status = Status::InProgress;
                tasks.insert(worker_entity, task.clone()).unwrap();
            }
        }

        // For each worker that has a task to complete
        for (worker_entity, _, task) in (&entities, &workers, &mut tasks).join() {
            match task
                .subtasks
                .iter_mut()
                .find(|t| t.status != Status::Completed)
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
                                Status::Actionable => {
                                    unreachable!();
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
                                Status::Actionable => {
                                    unreachable!();
                                }
                            }
                        }
                        _ => {}
                    }
                }
                None => {
                    dbg!("Nothing left to do, all the subtasks are completed");
                    tasks_to_unassign.push(worker_entity);
                }
            }
        }

        for entity in tasks_to_unassign {
            tasks.remove(entity).unwrap();
        }
    }
}
