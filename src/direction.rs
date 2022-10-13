use crate::point::Point;
use core::slice::Iter;


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn from_point(p: Point) -> Option<Direction> {
        match p {
            Point {x: 0, y: 1} => Some(Direction::Up),
            Point {x: 1, y: 0} => Some(Direction::Right),
            Point {x: 0, y: -1} => Some(Direction::Down),
            Point {x: -1, y: 0} => Some(Direction::Left),
            _ => None,
        }
    }

    pub fn point(self) -> Point{
        match self {
            Direction::Up => Point {x: 0, y: 1},
            Direction::Right => Point {x: 1, y: 0},
            Direction::Down => Point {x: 0, y: -1},
            Direction::Left => Point {x: -1, y: 0},
        }
    }

    pub fn invert(dir: Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
        DIRECTIONS.iter()
    }
}