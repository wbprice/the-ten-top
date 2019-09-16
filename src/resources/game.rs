use crate::resources::PatronOrder;

#[derive(Debug, Default)]
pub struct GameState {
    pub patron_orders: Vec<PatronOrder>,
    pub money: i32
}