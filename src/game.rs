use std::error::Error;
use crate::controller::Controller;
use crate::game_field::GameField;
use crate::player_action::PlayerAction;

struct Game<'a, Field: GameField> {
    field: Field,
    controllers: Vec<&'a mut dyn Controller>,
}

impl<'a, Field: GameField> Game<'a, Field>  {
    async fn run(&mut self) -> Result<i32, ()> {
        while self.field.winner().is_none() {
            let curr_player_num = self.field.current_player();
            let action = self.controllers[curr_player_num as usize].get_action();
            self.field.apply_action(action.await);
        }
        Ok(self.field.winner().unwrap())
    }
}