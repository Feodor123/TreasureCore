
use crate::array_2d::Array2D;
use crate::game_fields::rect_game_field::rect_game_field::RectGameField;
use crate::point::Point;
use crate::tile::Tile;
use crate::tiles::border_tile::BorderTile;
use crate::tiles::field_tile::FieldTile;
use crate::tiles::unreachable_tile::UnreachableTile;
use crate::tiles::unbreakable_wall_tile::UnbreakableWallTile;
use crate::topologies::rectangle_topology::RectangleTopology;


struct RectFieldBuilder {
    size: Point,
    //not counting walls and corners
    swamp_cluster_size: i32,
    swamp_cluster_count: i32,
    portal_count: i32,
    player_count: i32,
    river_drift: i32,
    ammo_max: i32,
}

impl RectFieldBuilder {
    fn generate(&self, _attempts: i32) -> Option<RectGameField> {
        let field = self.generate_empty();
        let arr: Vec<Box<dyn Tile>> = (0..self.size.volume()).map(|_x| Box::new(FieldTile {}) as Box<dyn Tile>).collect();
        let _canvas = RectangleTopology::new(Array2D::new(arr, self.size));
        Some(field)
    }

    fn generate_empty(&self) -> RectGameField {
        let true_size = self.size * 2 + Point { x: 1, y: 1 };
        let arr: Vec<Box<dyn Tile>> = (0..true_size.volume()).map(|_x| Box::new(BorderTile { wall: false }) as Box<dyn Tile>).collect();

        let mut tiles = Array2D::new(arr, true_size);

        for x in 0..true_size.x {
            for y in 0..true_size.y {
                if x % 2 == 0 && y % 2 == 0 {
                    tiles[Point { x, y }] = Box::new(UnreachableTile {});
                } else if x % 2 == 1 && y % 2 == 1 {
                    tiles[Point { x, y }] = Box::new(FieldTile {});
                }
            }
        }
        for x in (1..true_size.x).step_by(2) {
            tiles[Point { x, y: 0 }] = Box::new(UnbreakableWallTile {});
            tiles[Point { x, y: true_size.y - 1 }] = Box::new(UnbreakableWallTile {});
        }

        for y in (1..true_size.y).step_by(2) {
            tiles[Point { x: 0, y }] = Box::new(UnbreakableWallTile {});
            tiles[Point { x: true_size.x - 1, y }] = Box::new(UnbreakableWallTile {});
        }

        RectGameField::new(self.size, tiles, 0, vec![], vec![], None, self.ammo_max)
    }
    
    fn add_river(&self, _field: &mut RectGameField) -> bool {
        true
    }
}