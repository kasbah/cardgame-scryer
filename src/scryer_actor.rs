use actix::{Actor, Context, Handler, Message, System};

struct ScryerActor;

#[derive(Message)]
#[rtype(usize)]
struct Sum(usize, usize);

impl Actor for ScryerActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("I am alive!");
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("I am dead!");
    }
}

impl Handler<Sum> for ScryerActor {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Sum, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_actor() {
        let system = System::new();
        let addr = system.block_on(async { ScryerActor.start() });
        let res = system.block_on(async { addr.send(Sum(1, 2)).await.unwrap() });
        println!("Result: {:?}", res);
        system.run().unwrap();
    }
}
