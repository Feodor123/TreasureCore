use crate::direction::Direction;
use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, BulletMotionType, MotionInfo, MotionMessage, MotionType};
use crate::tile::Tile;

pub struct BorderTile {
    pub wall: bool,
}

impl Tile for BorderTile {
    fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        if self.wall {
            match motion_type {
                MotionType::Move(dir) => (MotionType::Move(Direction::invert(dir)), MotionInfo::Backtrack, MotionMessage::BlockedByWall),
                _ => panic!("Unexpected"),
            }
        } else {
            match motion_type {
                MotionType::Move(dir) => {
                    match motion_info {
                        MotionInfo::Move => (MotionType::Move(dir), MotionInfo::Move, MotionMessage::None),
                        MotionInfo::Backtrack => (MotionType::Move(dir), MotionInfo::Backtrack, MotionMessage::None),
                        _ => panic!("Unexpected MoveInfo variant"),
                    }
                }
                _ => panic!("Unexpected"),
            }
        }
    }

    fn on_shoot(&mut self, bullet_motion_type: BulletMotionType, bullet_motion_info: BulletMotionInfo) -> (BulletMotionType, BulletMotionInfo,
                                                                     BulletMotionMessage) {
        if self.wall {
            self.wall = false;
            (BulletMotionType::End, BulletMotionInfo::None, BulletMotionMessage::None)
        }
        else {
            match bullet_motion_type {
                BulletMotionType::Move(dir) => (BulletMotionType::Move(dir), BulletMotionInfo::None, BulletMotionMessage::None),
                _ => panic!("Unexpected"),
            }
        }
    }
}