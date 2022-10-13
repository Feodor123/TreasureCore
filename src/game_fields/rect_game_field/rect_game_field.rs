use std::ops::{Index, IndexMut};

use crate::array_2d::Array2D;
use crate::direction::Direction;
use crate::game_field::GameField;
use crate::misc_action_data::{BulletMotionInfo, BulletMotionMessage, BulletMotionType, MotionInfo, MotionMessage, MotionType};
use crate::player_action::PlayerAction::{self, Move, Shoot};
use crate::point::Point;
use crate::tile::Tile;

pub struct RectGameField {
    size: Point,
    tiles: Array2D<Box<dyn Tile>>,
    current_player: usize,
    characters: Vec<Character>,
    home_positions: Vec<Point>,
    treasure_pos: Option<Point>,
    ammo_max: i32,
}

impl GameField for RectGameField {
    fn size(&self) -> usize {
        (self.size.x * self.size.y) as usize
    }

    fn current_player(&self) -> usize {
        self.current_player
    }

    fn winner(&self) -> Option<i32> {
        for i in 0..self.characters.len() {
            if self.characters[i].has_treasure && self.characters[i].pos == self.home_positions[i] {
                return Some(i as i32);
            }
        }
        None
    }

    fn apply_action(&mut self, action: PlayerAction) -> Result<(), ()> {
        match action {
            Move(dir) => self.apply_move(dir),
            Shoot(dir) => self.apply_shot(dir),
        }
    }
}

impl Index<Point> for RectGameField {
    type Output = Box<dyn Tile>;

    fn index(&self, index: Point) -> &Self::Output {
        &self.tiles[index]
    }
}

impl IndexMut<Point> for RectGameField {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}

impl RectGameField {
    fn apply_move(&mut self, dir: Direction) -> Result<(), ()> {
        let mut mt = MotionType::Move(dir);
        let mut mi = MotionInfo::None;
        let mut mm = MotionMessage::None;
        while mt != MotionType::End {
            let new_pos = match mt {
                MotionType::Move(dir) => {
                    self.characters[self.current_player].pos + Direction::point(dir)
                }
                MotionType::Teleport { target_point } => {
                    target_point
                }
                _ => panic!("Unexpected")
            };
            self.move_character_to(self.current_player, new_pos);
            //TODO: rewrite when destructuring assignment will be stable
            let data = self[new_pos].on_step(mt, mi);
            mt = data.0;
            mi = data.1;
            mm = data.2;
            self.resolve_kills();
        }
        Ok(())
    }

    fn apply_shot(&mut self, dir: Direction) -> Result<(), ()> {
        if self.characters[self.current_player].ammo == 0 { return Err(()); }
        self.characters[self.current_player].ammo -= 1;
        let mut bmt = BulletMotionType::Move(dir);
        let mut bmi = BulletMotionInfo::None;
        let mut bmm = BulletMotionMessage::None;
        let mut pos = self.characters[self.current_player].pos;
        while bmt != BulletMotionType::End {
            pos = match bmt {
                BulletMotionType::Move(dir) => pos + dir,
                _ => panic!("Unexpected")
            };
            //TODO: rewrite when destructuring assignment will be stable
            let data = self[pos].on_shoot(bmt, bmi);
            bmt = data.0;
            bmi = data.1;
            bmm = data.2;

            for i in 0..self.characters.len() {
                if self.characters[i].pos == pos {
                    self.kill_character(i);
                    self.resolve_kills();
                    bmt = BulletMotionType::End;
                    break;
                }
            }
        }
        Ok(())
    }

    fn move_character_to(&mut self, character_num: usize, pos: Point) {
        self.characters[character_num].pos = pos;
        for i in 0..self.characters.len() {
            if i != character_num && !self.characters[i].killed && self.characters[i].pos == self.characters[character_num as usize].pos {
                self.kill_character(i);
                break;
            }
        }
        self.check_for_treasure(character_num);
        if self.characters[character_num].pos == self.home_positions[character_num] {
            self.characters[character_num].ammo = self.ammo_max;
        }
    }

    fn kill_character(&mut self, character_num: usize) {
        if self.characters[character_num].has_treasure {
            self.characters[character_num].has_treasure = false;
            self.treasure_pos = Some(self.characters[character_num].pos);
        }
        self.characters[character_num].killed = true;
    }

    fn resolve_kills(&mut self) {
        'inner: loop {
            for i in 0..self.characters.len() {
                if self.characters[i].killed {
                    self.characters[i].killed = false;
                    self.move_character_to(i, self.home_positions[i]);
                    continue 'inner;
                }
            }
        }
    }

    fn check_for_treasure(&mut self, character_num: usize) {
        if self.treasure_pos == Some(self.characters[character_num].pos) {
            self.characters[character_num].has_treasure = true;
            self.treasure_pos = None;
        }
    }
    pub fn new(size: Point, tiles: Array2D<Box<dyn Tile>>, current_player: usize, characters: Vec<Character>, home_positions: Vec<Point>, treasure_pos: Option<Point>, ammo_max: i32) -> Self {
        Self { size, tiles, current_player, characters, home_positions, treasure_pos, ammo_max }
    }
}

pub struct Character {
    pos: Point,
    ammo: i32,
    has_treasure: bool,
    killed: bool,
}