use crate::game_logic::GameState;
use actix::Message;

#[derive(Message)]
#[rtype(MoveChoice)]
pub struct MoveOptions {
    pub current: GameState,
    pub options: Vec<GameState>,
}

pub type MoveChoice = usize;
