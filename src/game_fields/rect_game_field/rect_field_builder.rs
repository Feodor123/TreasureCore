use rand::Rng;
use crate::array_2d::Array2D;
use crate::game_fields::rect_game_field::rect_game_field::RectGameField;
use crate::point::Point;
use crate::tile::Tile;
use crate::tiles::border_tile::BorderTile;
use crate::tiles::field_tile::FieldTile;
use crate::tiles::unreachable_tile::UnreachableTile;

struct RectFieldBuilder {
    size: Point,
    //not counting walls and corners
    swamp_cluster_size: i32,
    swamp_cluster_count: i32,
    portal_count: i32,
    player_count: i32,
    river_drift: i32,
    ammo_max: i32,
    bounded: bool,
}

impl RectFieldBuilder {
    fn generate(&self, attempts: i32) -> Option<RectGameField> {
        let mut field = self.generate_empty();
        Some(field)
    }

    fn generate_empty(&self) -> RectGameField {
        let mut arr: Vec<Box<dyn Tile>> = Vec::new();
        let true_size = if self.bounded { self.size * 2 + Point { x: 1, y: 1 } } else { self.size * 2 };
        for i in 0..true_size.x * true_size.y {
            arr.push(Box::new(BorderTile { wall: false }))
        }

        let tiles = Array2D::new(arr, true_size);

        for x in 0..true_size.x {
            for y in 0..true_size.y {
                if x % 2 == 0 && y % 2 == 0 {
                    tiles[Point { x, y }] = Box::new(UnreachableTile {});
                } else if x % 2 == 1 && y % 2 == 1 {
                    tiles[Point { x, y }] = Box::new(FieldTile {});
                }
            }
        }

        if self.bounded {
            for x in (1..true_size.x).step_by(2) {
                tiles[Point { x, y: 0 }] = Box::new(BorderTile { wall: false });
                tiles[Point { x, y: true_size.y - 1 }] = Box::new(BorderTile { wall: false });
            }

            for y in (1..true_size.y).step_by(2) {
                tiles[Point { x: 0, y }] = Box::new(BorderTile { wall: false });
                tiles[Point { x: true_size.x - 1, y }] = Box::new(BorderTile { wall: false });
            }
        }

        RectGameField::new(self.size, tiles, 0, vec![], vec![], None, 3)
    }

    // either add river to field and return true or do nothing and return false
    fn add_river(&self, field: &mut RectGameField) -> bool {
        let mut water = Array2D::<bool>::new(vec![false; (self.size.x * self.size.y) as usize], self.size);

        if self.bounded {
            let start = Point { x: rand::thread_rng().gen_range(0..self.size.x), y: self.size.y - 1 };
            
        } else {}
        false
    }
}