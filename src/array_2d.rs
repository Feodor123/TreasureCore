use std::ops::{Index, IndexMut};
use crate::point::Point;

pub struct Array2D<T> {
    arr: Vec<T>,
    size: Point,
}

impl<T> Array2D<T> {
    pub fn new(arr: Vec<T>, size: Point) -> Array2D<T> {
        Array2D {
            arr: arr,
            size: size,
        }
    }
}

impl<T> Array2D<T> {
    pub fn size(&self) -> Point {
        self.size
    }
}

impl<T> Index<Point> for Array2D<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.arr[(index.x + self.size.x * index.y) as usize]
    }
}

impl<T> IndexMut<Point> for Array2D<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.arr[(index.x + self.size.x * index.y) as usize]
    }
}

impl<T> IntoIterator for Array2D<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}