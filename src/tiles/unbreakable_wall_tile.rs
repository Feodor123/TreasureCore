use crate::direction::Direction;
use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, BulletMotionType, MotionInfo, MotionMessage, MotionType};
use crate::tile_logic::TileLogic;

#[derive(Clone)]
pub struct UnbreakableWallTile;

impl TileLogic for UnbreakableWallTile {
    fn on_step(&mut self, motion_type: MotionType, _motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        let dir = match motion_type {
            MotionType::Move(dir) => dir,
            _ => panic!("Unexpected MoveInfo variant"),
        };
        (MotionType::Move(Direction::invert(dir)), MotionInfo::Backtrack, MotionMessage::BlockedByWall)
    }

    fn on_shoot(&mut self, _bullet_motion_type: BulletMotionType, _bullet_motion_info: BulletMotionInfo) -> (BulletMotionType, BulletMotionInfo,
                                                                                                           BulletMotionMessage) {
        (BulletMotionType::End, BulletMotionInfo::None, BulletMotionMessage::None)
    }
}