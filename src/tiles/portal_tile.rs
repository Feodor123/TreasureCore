use crate::misc_action_data::{MotionInfo, MotionMessage, MotionType};
use crate::point::Point;
use crate::tile_logic::TileLogic;

#[derive(Clone)]
pub struct PortalTile {
    number: i32,
    next_portal_pos: Point,
}

impl TileLogic for PortalTile {
    fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        match motion_type {
            MotionType::Move(_dir) => {
                match motion_info {
                    MotionInfo::Move => (MotionType::Teleport { target_point: self.next_portal_pos }, MotionInfo::None, MotionMessage::EnterPortal { num: self.number }),
                    MotionInfo::Backtrack => (MotionType::End, MotionInfo::None, MotionMessage::None),
                    _ => panic!("Unexpected"),
                }
            }
            MotionType::Teleport { target_point: _ } => (MotionType::End, MotionInfo::None, MotionMessage::ExitPortal { num: self.number }),
            _ => panic!("Unexpected"),
        }
    }
}