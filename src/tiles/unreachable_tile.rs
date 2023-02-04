use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, BulletMotionType, MotionInfo, MotionMessage, MotionType};
use crate::tile_logic::TileLogic;

#[derive(Clone)]
pub struct UnreachableTile;

impl TileLogic for UnreachableTile {
    fn on_step(&mut self, _motion_type: MotionType, _motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        panic!("Unreachable tile");
    }

    fn on_shoot(&mut self, _bullet_motion_type: BulletMotionType, _bullet_motion_info: BulletMotionInfo) -> (BulletMotionType, BulletMotionInfo,
                                                                                                           BulletMotionMessage) {
        panic!("Unreachable tile");
    }
}