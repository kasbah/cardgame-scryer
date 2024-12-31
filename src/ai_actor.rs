use crate::move_messages::{MoveChoice, MoveOptions};
use crate::scryer_actor::ScryerActor;
use actix::{Actor, Addr, Context, Handler};
use actix_async_handler::async_handler;

pub struct AiActor {
    pub scryer: Addr<ScryerActor>,
}

impl Actor for AiActor {
    type Context = Context<Self>;
}

#[async_handler]
impl Handler<MoveOptions> for AiActor {
    type Result = MoveChoice;

    async fn handle(&mut self, _options: MoveOptions, _ctx: &mut Context<Self>) -> Self::Result {
        0
    }
}
