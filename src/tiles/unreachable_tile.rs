use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, BulletMotionType, MotionInfo, MotionMessage, MotionType};
use crate::tile::Tile;

pub struct UnreachableTile;

impl Tile for UnreachableTile {
    fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        panic!("Unreachable tile");
    }

    fn on_shoot(&mut self, bullet_motion_type: BulletMotionType, bullet_motion_info: BulletMotionInfo) -> (BulletMotionType, BulletMotionInfo,
                                                                                                           BulletMotionMessage) {
        panic!("Unreachable tile");
    }
}