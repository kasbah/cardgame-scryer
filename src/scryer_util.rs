use scryer_prolog::{LeafAnswer, Machine as ScryerMachine, Term};

pub fn query_once_binding(scryer: &mut ScryerMachine, query: &str, var: &str) -> Option<Term> {
    let mut answers = scryer.run_query(query);
    let answer = answers.next();
    match answer {
        Some(Ok(LeafAnswer::LeafAnswer { bindings, .. })) => match bindings.get(var) {
            Some(x) => Some(x.to_owned()),
            None => None,
        },
        _ => None,
    }
}
