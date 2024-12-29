use actix::{Actor, Context, Handler, Message, System};
use scryer_prolog::{Machine as ScryerMachine, MachineBuilder};

#[derive(Debug)]
pub struct ScryerActor {
    scryer: Option<ScryerMachine>,
}

#[derive(Message)]
#[rtype(usize)]
struct Sum(usize, usize);

impl Actor for ScryerActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let mut scryer = MachineBuilder::default().build();
        let file_content = include_str!("logic.pl");
        scryer.load_module_string("logic", file_content);
        self.scryer = Some(scryer);
        println!("I am alive!");
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("I am dead!");
    }
}

impl Handler<Sum> for ScryerActor {
    type Result = usize;

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
        let addr = system.block_on(async { ScryerActor { scryer : None }.start() });
        let res = system.block_on(async { addr.send(Sum(1, 2)).await.unwrap() });
        println!("Result: {:?}", res);
        system.run().unwrap();
    }
}
