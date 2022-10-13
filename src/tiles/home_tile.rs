use crate::misc_action_data::{MotionInfo, MotionMessage, MotionType};
use crate::tile::Tile;

pub struct HomeTile {
    number: i32,
}

impl Tile for HomeTile {
    fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        match motion_type {
            MotionType::Move(_dir) => {
                match motion_info {
                    MotionInfo::Move => (MotionType::End, MotionInfo::None, MotionMessage::Home {num: self.number}),
                    MotionInfo::Backtrack => (MotionType::End, MotionInfo::None, MotionMessage::None),
                    _ => panic!("Unexpected"),
                }
            },
            _ => panic!("Unexpected"),
        }
    }
}