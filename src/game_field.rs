use crate::player_action::PlayerAction;


// information about playing field (including player's positions) and its update logic
pub trait GameField {
    //size, including walls and corners
    fn size(&self) -> usize;

    fn current_player(&self) -> usize;

    fn winner(&self) -> Option<i32>;

    fn apply_action(&mut self, action: PlayerAction) -> Result<(), ()>;
}