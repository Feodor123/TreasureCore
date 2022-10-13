use std::ops;
use crate::direction::Direction;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

impl ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, dir: Direction) -> Point {
        self + Direction::point(dir)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, p: Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Point {
    pub fn volume(&self) -> i32 {
        self.x*self.y
    }
    
    fn sinmul(p1: Point, p2: Point) -> i32 {
        p1.x * p2.y - p2.x * p1.y
    }

    fn collinear(p1: Point, p2: Point) -> bool {
        Point::sinmul(p1, p2) == 0
    }
}

pub struct ModPointComparator {
    mod1: Point,
    // mod1.x must always be non-zero
    mod2: Point,
    // mod2.y must always be non-zero
    volume: i32, // positive
}

impl ModPointComparator {
    //mod1 and mod2 must not be collinear
    pub fn new(mod1: Point, mod2: Point) -> ModPointComparator {
        assert!(!Point::collinear(mod1, mod2));
        let volume = i32::abs(Point::sinmul(mod1, mod2));
        if mod1.x != 0 && mod2.y != 0 {
            ModPointComparator { mod1, mod2, volume }
        } else {
            ModPointComparator { mod2, mod1, volume }
        }
    }

    pub fn equal(&self, p1: Point, p2: Point) -> bool {
        let p3 = p1 - p2;
        return Point::sinmul(p3, self.mod1) % self.volume == 0 && Point::sinmul(p3, self.mod2) % self.volume == 0;
    }
}