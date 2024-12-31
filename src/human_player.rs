use crate::move_messages::{MoveChoice, MoveRequest};
use actix::{Actor, Addr, Context, Handler};
use actix_async_handler::async_handler;

pub struct HumanPlayer {
}

impl Actor for HumanPlayer {
    type Context = Context<Self>;
}

#[async_handler]
impl Handler<MoveRequest> for HumanPlayer {
    type Result = MoveChoice;

    async fn handle(&mut self, _options: MoveRequest, _ctx: &mut Context<Self>) -> Self::Result {
        0
    }
}
