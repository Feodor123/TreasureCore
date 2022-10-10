use crate::player_action::PlayerAction;
use async_trait::async_trait;

#[async_trait]
pub trait Controller {
    async fn get_action(&mut self) -> PlayerAction;
}