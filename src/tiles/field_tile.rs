use crate::misc_action_data::{MotionInfo, MotionMessage, MotionType};
use crate::tile_logic::TileLogic;

#[derive(Clone)]
pub struct FieldTile;

impl TileLogic for FieldTile {
    fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        match motion_type {
            MotionType::Move(_dir) => {
                match motion_info {
                    MotionInfo::Move => (MotionType::End, MotionInfo::None, MotionMessage::Field),
                    MotionInfo::Backtrack => (MotionType::End, MotionInfo::None, MotionMessage::None),
                    _ => panic!("Unexpected"),
                }
            },
            _ => panic!("Unexpected"),
        }
    }
}