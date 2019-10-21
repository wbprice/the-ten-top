use amethyst::{
    core::transform::{Parent, Transform},
    ecs::prelude::{Entities, Entity, Join, ReadStorage, System, Write, WriteStorage},
};

use crate::{
    components::{Destination, Emotion, Feeling, Food, Patron, Subtask, Task, Worker},
    resources::{GameState, Status, Subtasks, Tasks},
};

pub struct PatronTaskSystem;

fn get_distance_between_two_points(point_a: [f32; 2], point_b: [f32; 2]) -> f32 {
    let x = point_a[0] - point_b[0];
    let y = point_a[1] - point_b[1];
    let c: f32 = x.powi(2) + y.powi(2);
    c.sqrt()
}

impl<'s> System<'s> for PatronTaskSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Patron>,
        ReadStorage<'s, Worker>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Food>,
        WriteStorage<'s, Destination>,
        WriteStorage<'s, Task>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, Feeling>,
        Write<'s, GameState>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut patrons,
            workers,
            mut locals,
            foods,
            mut destinations,
            mut tasks,
            mut parents,
            mut feelings,
            mut game_state,
        ): Self::SystemData,
    ) {
        let mut tasks_to_remove: Vec<Entity> = vec![];

        // Perform first time setup when the patron is handed a task.
        for (patron_entity, patron, task) in (&entities, &patrons, &mut tasks)
            .join()
            .filter(|(_, _, task)| task.status == Status::New)
        {
            // Populate task subactivities based on type of activity.
            match task.activity {
                Tasks::GiveOrder { register, food } => {
                    task.subtasks
                        .push(Subtask::new(Subtasks::MoveToEntity { entity: register }));
                    task.subtasks.push(Subtask::new(Subtasks::WaitForWorker));
                    task.subtasks
                        .push(Subtask::new(Subtasks::SubmitOrder { food }));
                    task.subtasks
                        .push(Subtask::new(Subtasks::WaitForOrder { food }));
                    task.subtasks.push(Subtask::new(Subtasks::UpdateFeeling {
                        symbol: Emotion::Happy,
                    }));
                    task.subtasks.push(Subtask::new(Subtasks::MoveTo {
                        destination: Destination { x: 144.0, y: 30.0 },
                    }));
                }
                _ => {
                    unimplemented!();
                }
            }
        }

        // Start handling task behaviors.
        for (patron_entity, _, task) in (&entities, &patrons, &mut tasks).join() {
            task.status = Status::InProgress;

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
                                    dbg!("[MoveToEntity] starting the MoveToEntity task");
                                    // Where is the entity to walk to?
                                    let entity_local = locals.get(entity).unwrap();
                                    let entity_transform = entity_local.translation();

                                    // Direct the worker to walk to the entity
                                    destinations
                                        .insert(
                                            patron_entity,
                                            Destination {
                                                x: entity_transform.x,
                                                y: entity_transform.y,
                                            },
                                        )
                                        .unwrap();

                                    subtask.status = Status::InProgress;
                                }
                                Status::InProgress => {
                                    dbg!("[MoveToEntity] patron is en route");
                                    // Destination storage will remove the destination
                                    // once the entity has reached it's destination.
                                    // So the destination no longer exists, we can call
                                    // the task done.
                                    if let None = destinations.get(patron_entity) {
                                        dbg!("[MoveToEntity] patron has arrived at their destination");
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
                        Subtasks::WaitForWorker => {
                            match subtask.status {
                                Status::New => {
                                    dbg!("[WaitForWorker] starting the WaitForWorker task for a patron");
                                    game_state.schedule_take_order(patron_entity);
                                    subtask.status = Status::InProgress;
                                }
                                Status::InProgress => {
                                    dbg!("[WaitForWorker] worker summoned, waiting for them to arrive");
                                    // Check to see if a worker has approached the register.
                                    // If so, mark the subtask completed.
                                    let patron_local = locals.get(patron_entity).unwrap();
                                    let patron_loc = patron_local.translation();
                                    let patron_x = patron_loc.x;
                                    let patron_y = patron_loc.y;
                                    for (_, worker_local) in (&workers, &locals).join() {
                                        let worker_loc = worker_local.translation();
                                        let worker_x = worker_loc.x;
                                        let worker_y = worker_loc.y;
                                        let dist = get_distance_between_two_points(
                                            [worker_x, worker_y],
                                            [patron_x, patron_y],
                                        );
                                        if dist < 8.0 {
                                            dbg!("[WaitForWorker] worker has arrived");
                                            subtask.status = Status::Completed;
                                        }
                                    }
                                }
                                Status::Completed => {
                                    unreachable!();
                                }
                                Status::Blocked => {
                                    unimplemented!("[WaitForWorker] blocked tasks haven't been implemented yet!");
                                }
                            }
                        }
                        Subtasks::SubmitOrder { food } => {
                            match subtask.status {
                                Status::New => {
                                    dbg!("[SubmitOrder] starting task");
                                    // Display speech bubble component for 3 seconds.
                                    subtask.status = Status::InProgress;
                                }
                                Status::InProgress => {
                                    // Hide the speech bubble.
                                    dbg!("[SubmitOrder] ordering dish");
                                    game_state.schedule_deliver_order(patron_entity, food);
                                    subtask.status = Status::Completed;
                                    dbg!("[SubmitOrder] dish ordered");
                                }
                                Status::Completed => {
                                    unreachable!();
                                }
                                Status::Blocked => {
                                    unimplemented!("[SubmitOrder] blocked is unimplemented");
                                }
                            }
                        }
                        Subtasks::WaitForOrder { food } => {
                            match subtask.status {
                                Status::New => {
                                    dbg!("[WaitForOrder] task started");
                                    subtask.status = Status::InProgress;
                                }
                                Status::InProgress => {
                                    dbg!("[WaitForOrder] waiting for food to be delivered");
                                    // Wait until the patron entity has a dish of that type
                                    for (food, parent) in (&foods, &parents).join() {
                                        if parent.entity == patron_entity {
                                            dbg!("[WaitForOrder] food recieved");
                                            subtask.status = Status::Completed;
                                        }
                                    }
                                }
                                Status::Completed => {
                                    unreachable!();
                                }
                                Status::Blocked => {
                                    unimplemented!(
                                        "[WaitForOrder] blocked tasks haven't been implemented yet"
                                    );
                                }
                            }
                        }
                        Subtasks::MoveTo { destination } => {
                            match subtask.status {
                                Status::New => {
                                    dbg!("[MoveTo] starting the MoveTo task");

                                    // Direct the patron to walk to the entity
                                    destinations.insert(patron_entity, destination).unwrap();

                                    subtask.status = Status::InProgress;
                                }
                                Status::InProgress => {
                                    dbg!("[MoveTo] patron is en route");
                                    // Destination storage will remove the destination
                                    // once the entity has reached it's destination.
                                    // So the destination no longer exists, we can call
                                    // the task done.
                                    if let None = destinations.get(patron_entity) {
                                        dbg!("[MoveTo] patron has arrived at their destination");
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
                        Subtasks::UpdateFeeling { symbol } => {
                            feelings.insert(patron_entity, Feeling { symbol }).unwrap();
                            subtask.status = Status::Completed;
                        }
                        _ => {
                            unimplemented!();
                        }
                    }
                }
                None => {
                    dbg!("No subtasks remaining");
                    tasks_to_remove.push(patron_entity);
                }
            }
        }

        // Perform any cleanup.
        for entity in tasks_to_remove {
            tasks.remove(entity).unwrap();
            dbg!("Patron tasks removed, free for reassignment");
        }
    }
}
