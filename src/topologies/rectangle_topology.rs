use std::ops::{Index, IndexMut};
use crate::array_2d::Array2D;
use crate::direction::Direction;
use crate::point::Point;
use crate::topology::Topology;

struct RectangleTopology<T> {
    arr: Array2D<T>,
}

impl<T> Index<Point> for RectangleTopology<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.arr[index]
    }
}

impl<T> IndexMut<Point> for RectangleTopology<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.arr[index]
    }
}

impl<T> IntoIterator for RectangleTopology<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

impl<T> Topology<T> for RectangleTopology<T> {
    fn neighbours(&self, p: Point) -> Vec<Point> {
        let mut vec = Vec::new();
        for &dir in Direction::iter() {
            let pp = p + dir;
            if pp.x >= 0 && pp.x < self.arr.size().x && pp.y >= 0 && pp.y < self.arr.size().y {
                vec.push(pp);
            }
        }
        vec
    }

    fn normalize(&self, p: Point) -> Point {
        p
    }
}