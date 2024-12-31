use crate::move_messages::{MoveChoice, MoveOptions};
use actix::{Actor, Addr, Context, Handler};
use actix_async_handler::async_handler;

pub struct PlayerActor {
}

impl Actor for PlayerActor {
    type Context = Context<Self>;
}

#[async_handler]
impl Handler<MoveOptions> for PlayerActor {
    type Result = MoveChoice;

    async fn handle(&mut self, _options: MoveOptions, _ctx: &mut Context<Self>) -> Self::Result {
        0
    }
}
