use crate::array_2d::Array2D;
use crate::game_fields::rect_game_field::rect_game_field::RectGameField;
use crate::point::Point;
use crate::tile::Tile;
use crate::tiles::border_tile::BorderTile;
use crate::tiles::field_tile::FieldTile;
use crate::tiles::unreachable_tile::UnreachableTile;

struct RectFieldBuilder {
    size: Point,
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
    }

    fn generate_empty(&self) -> RectGameField {
        let mut arr: Vec<Box<dyn Tile>> = Vec::new();
        for i in 0..self.size.x * self.size.y * 4 {
            arr.push(Box::new(BorderTile { wall: false }))
        }

        let tiles = Array2D::new(arr, self.size * 2);

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                tiles[Point { x: 2 * x, y: 2 * y }] = Box::new(FieldTile {});
                tiles[Point { x: 2 * x + 1, y: 2 * y + 1 }] = Box::new(UnreachableTile {});
            }
        }

        RectGameField::new(self.size, tiles, 0, vec![], vec![], None, 3)
    }

    fn add_river(&self, &mut field) {

    }
}