use crate::scryer_actor::{QueryOnce, ScryerActor};
use actix::Addr;
use scryer_prolog::{LeafAnswer, Term};

pub async fn query_once_binding(scryer: &Addr<ScryerActor>, query: &str, var: &str) -> Option<Term> {
    let answer = scryer
        .send(QueryOnce(query.to_string()))
        .await;
    match answer {
        Ok(Ok(LeafAnswer::LeafAnswer { bindings, .. })) => bindings.get(var).cloned(),
        _ => None,
    }
}
