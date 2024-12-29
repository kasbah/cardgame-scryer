use actix::{Actor, Context, System, Message, Handler};
use scryer_prolog::{Machine as ScryerMachine, MachineBuilder};

pub struct ScryerActor {
    scryer: ScryerMachine,
}

#[derive(Message)]
#[rtype(usize)]
struct Query(String);


impl Actor for ScryerActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.scryer = MachineBuilder::default().build();
        let file_content = include_str!("logic.pl");
        self.scryer.load_module_string("logic", file_content);
        println!("Scryer actor started");
    }

}

impl Handler<Query> for ScryerActor {
    type Result = usize;

    fn handle(&mut self, msg: Query, _ctx: &mut Self::Context) -> Self::Result {
        let query = msg.0;
        let answers = self.scryer.run_query(&query);
        answers.count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let system = System::new();
        let addr = system.block_on(async { ScryerActor.start() });
        addr.send("card(Id, Distance, Temp, OrbitTime, Radius, Mass, EarthSimilarity)").unwrap();
    }
}
