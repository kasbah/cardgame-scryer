use actix::{Actor, Handler, Message, SyncArbiter, SyncContext, System};
use scryer_prolog::{LeafAnswer, Machine as ScryerMachine, MachineBuilder, Term};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ScryerActor {
    scryer: ScryerMachine,
}

pub type QueryResult = Vec<BTreeMap<String, Term>>;

#[derive(Message)]
#[rtype(QueryResult)]
struct Query(String);

impl Actor for ScryerActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("I am alive!");
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("I am dead!");
    }
}

impl Handler<Query> for ScryerActor {
    type Result = QueryResult;

    fn handle(&mut self, query: Query, _ctx: &mut SyncContext<Self>) -> Self::Result {
        self.scryer
            .run_query(&query.0)
            .filter_map(|answer| match answer {
                Ok(LeafAnswer::LeafAnswer { bindings, .. }) => Some(bindings),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_actor() {
        let system = System::new();
        let addr = system.block_on(async {
            SyncArbiter::start(1, || {
                let mut scryer = MachineBuilder::default().build();
                let file_content = include_str!("logic.pl");
                scryer.load_module_string("logic", file_content);
                ScryerActor { scryer }
            })
        });
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
