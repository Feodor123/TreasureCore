use rand::Rng;
use rand::seq::SliceRandom;
use crate::direction::Direction;
use crate::point::Point;
use crate::tile::Tile;
use crate::tiles::water_tile::WaterTile;
use crate::topologies::rectangle_topology::RectangleTopology;
use crate::topology::Topology;

pub struct FiniteRiverBuilder {}

impl FiniteRiverBuilder {
    pub fn add_river(canvas: &mut RectangleTopology<Box<dyn Tile>>, drift: i32) {
        let mut curr_point = Point { x: rand::thread_rng().gen_range(0..canvas.size().x), y: canvas.size().y - 1 };
        let mut points = Vec::new();
        while curr_point.y >= 0 {
            points.push(curr_point);
            let mut possible_next: Vec<Point> = canvas.neighbours(curr_point).into_iter().
                filter(|&p| p.y <= curr_point.y &&
                    !(points.len() >= 2 && points[points.len() - 2] == p) &&
                !(points.len() >= 3 && points[points.len() - 3] == (p + Direction::Up))).collect();
            if curr_point.y == 0 {
                possible_next.push(curr_point + Direction::Down);
            }
            curr_point = *possible_next.choose(&mut rand::thread_rng()).unwrap();
        }
        for i in 0..points.len() - 1 {
            canvas[points[i]] = Box::new(WaterTile { flow_direction: Direction::from_point(points[i+1] - points[i]).unwrap(), drift});
        }
        canvas[*points.last().unwrap()] = Box::new(WaterTile { flow_direction: Direction::Down, drift});
    }
}