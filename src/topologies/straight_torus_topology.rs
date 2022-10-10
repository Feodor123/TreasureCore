use std::ops::{Index, IndexMut};
use crate::array_2d::Array2D;
use crate::direction::Direction;
use crate::point::Point;
use crate::topology::Topology;

struct StraightTorusTopology<T> {
    arr: Array2D<T>,
}

impl<T> Index<Point> for StraightTorusTopology<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.arr[index]
    }
}

impl<T> IndexMut<Point> for StraightTorusTopology<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.arr[index]
    }
}

impl<T> IntoIterator for StraightTorusTopology<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

impl<T> Topology<T> for StraightTorusTopology<T> {
    fn neighbours(&self, p: Point) -> Vec<Point> {
        Direction::iter().map(|dir| self.normalize(p + *dir)).collect()
    }

    fn normalize(&self, p: Point) -> Point {
        Point {x: p.x % self.arr.size().x, y: p.y % self.arr.size().y }
    }
}