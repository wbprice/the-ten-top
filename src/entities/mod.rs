mod cupboard;
mod dish;
mod feeling;
mod ingredient;
mod patron;
mod plate;
mod register;
mod stove;
mod thought_bubble;
mod worker;

pub use self::{
    cupboard::init_cupboard, dish::init_dish, feeling::init_feeling, ingredient::init_ingredient,
    patron::init_patron, plate::init_plate, register::init_register, stove::init_stove,
    thought_bubble::init_thought_bubble, worker::init_worker,
};
