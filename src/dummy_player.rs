use crate::move_request::{MoveChoice, MoveRequest};
use actix::{Actor, Context, Handler};

pub struct DummyPlayer {}

impl Actor for DummyPlayer {
    type Context = Context<Self>;
}

impl Handler<MoveRequest> for DummyPlayer {
    type Result = MoveChoice;

    fn handle(&mut self, _options: MoveRequest, _ctx: &mut Context<Self>) -> Self::Result {
        0
    }
}
