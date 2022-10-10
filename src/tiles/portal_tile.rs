use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, MotionInfo, MotionMessage, MotionType};
use crate::point::Point;
use crate::tile::Tile;

pub struct PortalTile {
    number: i32,
    next_portal_pos: Point,
}

impl Tile for PortalTile {
    fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        match motion_type {
            MotionType::Move(dir) => {
                match motion_info {
                    MotionInfo::Move => (MotionType::Teleport { target_point: self.next_portal_pos }, MotionInfo::None, MotionMessage::EnterPortal { num: self.number }),
                    MotionInfo::Backtrack => (MotionType::End, MotionInfo::None, MotionMessage::None),
                    _ => panic!("Unexpected"),
                }
            }
            MotionType::Teleport { target_point } => (MotionType::End, MotionInfo::None, MotionMessage::ExitPortal { num: self.number }),
            _ => panic!("Unexpected"),
        }
    }
}