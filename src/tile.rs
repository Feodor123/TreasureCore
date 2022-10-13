use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, BulletMotionType, MotionInfo, MotionMessage, MotionType};

pub trait Tile {
    fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage);

    fn on_shoot(&mut self, bullet_motion_type: BulletMotionType, _bullet_motion_info: BulletMotionInfo) -> (BulletMotionType, BulletMotionInfo, BulletMotionMessage) {
        match bullet_motion_type {
            BulletMotionType::Move(dir) => (BulletMotionType::Move(dir), BulletMotionInfo::None, BulletMotionMessage::None),
            _ => panic!("Unexpected"),
        }
    }
}