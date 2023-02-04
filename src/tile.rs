use enum_dispatch::enum_dispatch;
use crate::tiles::tiles::Tiles;
use crate::tiles::border_tile::BorderTile;
use crate::tiles::field_tile::FieldTile;
use crate::tiles::home_tile::HomeTile;
use crate::tiles::portal_tile::PortalTile;
use crate::tiles::swamp_tile::SwampTile;
use crate::tiles::unbreakable_wall_tile::UnbreakableWallTile;
use crate::tiles::unreachable_tile::UnreachableTile;
use crate::tiles::water_tile::WaterTile;
use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, BulletMotionType, MotionInfo, MotionMessage, MotionType};

#[enum_dispatch]
pub trait Tile
{
   fn on_step(&mut self, motion_type: MotionType, motion_info: MotionInfo) -> (MotionType, MotionInfo, MotionMessage);

   fn on_shoot(&mut self, bullet_motion_type: BulletMotionType, _bullet_motion_info: BulletMotionInfo) -> (BulletMotionType, BulletMotionInfo, BulletMotionMessage) {
        match bullet_motion_type {
            BulletMotionType::Move(dir) => (BulletMotionType::Move(dir), BulletMotionInfo::None, BulletMotionMessage::None),
            _ => panic!("Unexpected"),
        }
    }
}