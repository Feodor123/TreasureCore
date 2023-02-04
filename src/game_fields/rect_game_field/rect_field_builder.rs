use crate::array_2d::Array2D;
use crate::game_fields::rect_game_field::finite_river_builder::FiniteRiverBuilder;
use crate::game_fields::rect_game_field::rect_game_field::RectGameField;
use crate::point::Point;
use crate::tile::Tile;
use crate::tiles::border_tile::BorderTile;
use crate::tiles::field_tile::FieldTile;
use crate::tiles::unreachable_tile::UnreachableTile;
use crate::tiles::unbreakable_wall_tile::UnbreakableWallTile;
use crate::topologies::rectangle_topology::RectangleTopology;


pub struct RectFieldBuilder {
    pub size: Point,
    //not counting walls and corners
    pub swamp_cluster_size: i32,
    pub swamp_cluster_count: i32,
    pub portal_count: i32,
    pub player_count: i32,
    pub river_drift: i32,
    pub ammo_max: i32,
}

impl RectFieldBuilder {
    pub fn generate(&self, _attempts: i32) -> Option<RectGameField> {
        let mut field = self.generate_empty();
        let arr: Vec<Box<dyn Tile>> = (0..self.size.volume()).map(|_x| Box::new(FieldTile {}) as Box<dyn Tile>).collect();
        let mut canvas = RectangleTopology::new(Array2D::new(arr, self.size));
        FiniteRiverBuilder::add_river(&mut canvas, self.river_drift);
        self.apply_canvas(&mut field, &mut canvas);
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

        RectGameField::new(true_size, tiles, 0, vec![], vec![], None, self.ammo_max)
    }
    
    fn add_river(&self, _field: &mut RectGameField) -> bool {
        true
    }

    fn apply_canvas(&self, field: &mut RectGameField, canvas: &mut RectangleTopology<Box<dyn Tile>>) {
        assert!(field.shape() == canvas.size()*2 + Point {x:1, y:1});
        for x in 0..canvas.size().x {
            for y in 0..canvas.size().y {
                field[Point{x: 2*x+1, y: 2*y+1}] = canvas[Point{x, y}].clone();
            }
        }
    }
}