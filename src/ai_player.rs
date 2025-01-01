use crate::move_request::{MoveChoice, MoveRequest};
use crate::scryer_actor::ScryerActor;
use actix::{Actor, Context, Handler};
use actix_async_handler::async_handler;

pub struct AiPlayer {}

impl Actor for AiPlayer {
    type Context = Context<Self>;
}

#[async_handler]
impl Handler<MoveRequest> for AiPlayer {
    type Result = MoveChoice;

    async fn handle(&mut self, _options: MoveRequest, _ctx: &mut Context<Self>) -> Self::Result {
        0
    }
}
