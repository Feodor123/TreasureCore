use crate::direction::Direction;
use crate::point::Point;

//state for movement between tiles logic
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum MotionType {
    Move(Direction),
    Teleport { target_point: Point },
    End,
}

//state for tile logic
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum MotionInfo {
    None,
    Move,
    Flow {drift_left: i32},
    Backtrack,
}

//state for messages about visited tiles
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

//state for bullet movement 
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BulletMotionType {
    Move(Direction),
    End,
}

//state for tile logic when shooting
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BulletMotionInfo {
    None
}

//state for messages about shooting
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BulletMotionMessage {
    None,
}