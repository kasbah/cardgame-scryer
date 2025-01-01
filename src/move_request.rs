use crate::game_logic::GameState;
use actix::Message;

#[derive(Message)]
#[rtype(MoveChoice)]
pub struct MoveRequest {
    pub visible_state: GameState,
    pub options: Vec<GameState>,
}

pub type MoveChoice = usize;
