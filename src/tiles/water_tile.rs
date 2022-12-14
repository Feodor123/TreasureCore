use crate::direction::Direction;
use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, MotionInfo, MotionMessage, MotionType};
use crate::tile::Tile;

pub struct WaterTile {
    flow_direction: Direction,
    drift: i32,
}

impl Tile for WaterTile {
    fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage) {
        match motion_type {
            MotionType::Move(dir) => {},
            _ => panic!("Unexpected"),
        };
        match motion_info {
            MotionInfo::Move => (MotionType::Move(self.flow_direction), MotionInfo::Flow {drift_left: self.drift - 1}, MotionMessage::Water),
            MotionInfo::Flow { drift_left} => {
                if drift_left == 0 {
                    (MotionType::End, MotionInfo::None, MotionMessage::Water)
                }
                else {
                    (MotionType::Move(self.flow_direction), MotionInfo::Flow { drift_left: drift_left - 1}, MotionMessage::Water)
                }
            },
            MotionInfo::Backtrack => (MotionType::End, MotionInfo::None, MotionMessage::None),
            _ => panic!("Unexpected MoveInfo variant"),
        }
    }
}