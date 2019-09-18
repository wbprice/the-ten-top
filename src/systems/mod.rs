mod assignment;
mod destination;
mod feeling;
mod food;
mod patron;
mod register;
mod simple_animation;
mod thought_bubble;
mod worker;

pub use self::{
    assignment::AssignmentSystem, destination::DestinationSystem, feeling::MoveFeelingSystem,
    food::FoodSystem, patron::MovePatronSystem, register::RegisterSystem,
    simple_animation::SimpleAnimationSystem, thought_bubble::MoveThoughtBubbleSystem,
    worker::WorkerSystem,
};
