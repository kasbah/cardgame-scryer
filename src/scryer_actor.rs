use actix::{Actor, Context, Handler, Message, System};
use scryer_prolog::{Machine as ScryerMachine, MachineBuilder};

#[derive(Debug)]
pub struct ScryerActor {
    scryer: Option<ScryerMachine>,
}

#[derive(Message)]
#[rtype(usize)]
struct Query(String);

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

impl Handler<Query> for ScryerActor {
    type Result = usize;

    fn handle(&mut self, query: Query, _ctx: &mut Context<Self>) -> Self::Result {
        let scryer = self.scryer.as_mut().unwrap();
        let answers = scryer.run_query(&query.0);
        answers.count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_actor() {
        let system = System::new();
        let addr = system.block_on(async { ScryerActor { scryer: None }.start() });
        let res = system.block_on(async {
            addr.send(Query(
                "card(Id, Distance, Temp, OrbitTime, Radius, Mass, EarthSimilarity).".to_string(),
            ))
            .await
            .unwrap()
        });
        println!("Result: {:?}", res);
        system.run().unwrap();
    }
}
