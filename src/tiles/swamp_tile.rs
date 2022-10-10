use crate::direction::Direction;
use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, MotionInfo, MotionMessage, MotionType};
use crate::tile::Tile;

pub struct SwampTile;

impl Tile for SwampTile {
    fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        let dir = match motion_type {
            MotionType::Move(dir) => dir,
            _ => panic!("Unexpected"),
        };
        (MotionType::Move(Direction::invert(dir)), MotionInfo::Backtrack, MotionMessage::Swamp)
    }
}