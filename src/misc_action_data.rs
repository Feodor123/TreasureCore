use crate::direction::Direction;
use crate::point::Point;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum MotionType {
    Move(Direction),
    Teleport { target_point: Point },
    End,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum MotionInfo {
    None,
    Move,
    Flow {drift_left: i32},
    Backtrack,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum MotionMessage {
    None,
    Field,
    Swamp,
    Water,
    BlockedByWall,
    BlockedByGrate,
    EnterPortal { num: i32 },
    ExitPortal { num: i32 },
    Home { num: i32 },
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BulletMotionType {
    Move(Direction),
    End,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BulletMotionInfo {
    None
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BulletMotionMessage {
    None,
}