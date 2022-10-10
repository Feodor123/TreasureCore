use std::ops::{Index, IndexMut};
use crate::array_2d::Array2D;
use crate::direction::Direction;
use crate::point::Point;

pub trait Topology<T>: Index<Point> + IndexMut<Point> + IntoIterator {
    fn neighbours(&self, p: Point) -> Vec<Point>;

    fn normalize(&self, p: Point) -> Point;
}
