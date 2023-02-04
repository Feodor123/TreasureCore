pub mod game_field;
pub mod misc_action_data;
pub mod direction;
pub mod point;
pub mod tiles;
pub mod tile;
pub mod game_fields;
pub mod game;
pub mod player_action;
pub mod controller;
pub mod array_2d;
pub mod topology;
pub mod topologies;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
