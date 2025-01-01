use actix::{Actor, Handler, Message, SyncContext};
use scryer_prolog::{LeafAnswer, Machine as ScryerMachine, Term};

#[derive(Debug)]
pub struct ScryerActor {
    scryer: ScryerMachine,
}
impl Actor for ScryerActor {
    type Context = SyncContext<Self>;
}


#[derive(Message)]
#[rtype(QueryResult)]
struct Query(String);

pub type QueryResult = Vec<Result<LeafAnswer, Term>>;

impl Handler<Query> for ScryerActor {
    type Result = QueryResult;

    fn handle(&mut self, query: Query, _ctx: &mut SyncContext<Self>) -> Self::Result {
        self.scryer.run_query(&query.0).collect()
    }
}


#[derive(Message)]
#[rtype(QueryOnceResult)]
struct QueryOnce(String);

pub type QueryOnceResult = Result<LeafAnswer, Term>;

impl Handler<QueryOnce> for ScryerActor {
    type Result = QueryOnceResult;

    fn handle(&mut self, query: QueryOnce, _ctx: &mut SyncContext<Self>) -> Self::Result {
        self.scryer.run_query(&query.0).next().expect("No result from QueryOnce")
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use actix::{Addr, SyncArbiter, System};
    use scryer_prolog::MachineBuilder;

    #[test]
    fn test_actor() {
        let system = System::new();
        let addr: Addr<ScryerActor> = system.block_on(async {
            SyncArbiter::start(1, || {
                let mut scryer = MachineBuilder::default().build();
                let file_content = include_str!("logic.pl");
                scryer.load_module_string("logic", file_content);
                ScryerActor { scryer }
            })
        });
        let _res1 = system.block_on(async {
            addr.send(Query(
                "card(Id, Distance, Temp, OrbitTime, Radius, Mass, EarthSimilarity).".to_string(),
            ))
            .await
            .unwrap()
        });
        let _res2 = system.block_on(async {
            addr.send(QueryOnce(
                "card(Id, Distance, Temp, OrbitTime, Radius, Mass, EarthSimilarity).".to_string(),
            ))
            .await
            .unwrap()
        });
        System::current().stop();
    }
}
