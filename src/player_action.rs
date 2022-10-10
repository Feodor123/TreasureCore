use crate::direction::Direction;

pub enum PlayerAction {
    Move(Direction),
    Shoot(Direction),
}