mod game_field;
mod misc_action_data;
mod direction;
mod point;
mod tiles;
pub mod tile;
mod game_fields;
mod game;
mod player_action;
mod controller;
mod array_2d;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
