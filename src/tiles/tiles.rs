use enum_dispatch::enum_dispatch;
use crate::tiles::border_tile::BorderTile;
use crate::tiles::field_tile::FieldTile;
use crate::tiles::home_tile::HomeTile;
use crate::tiles::portal_tile::PortalTile;
use crate::tiles::swamp_tile::SwampTile;
use crate::tiles::unbreakable_wall_tile::UnbreakableWallTile;
use crate::tiles::unreachable_tile::UnreachableTile;
use crate::tiles::water_tile::WaterTile;

#[enum_dispatch(Tile)]
pub enum Tiles {
    BorderTile,
    FieldTile,
    HomeTile,
    PortalTile,
    SwampTile,
    UnbreakableWallTile,
    UnreachableTile,
    WaterTile,
}