use scryer_prolog::{LeafAnswer, Machine as ScryerMachine, Term};

pub fn query_once_binding(scryer: &mut ScryerMachine, query: &str, var: &str) -> Option<Term> {
    let mut answers = scryer.run_query(query);
    let answer = answers.next();
    match answer {
        Some(Ok(LeafAnswer::LeafAnswer { bindings, .. })) => bindings.get(var).cloned(),
        _ => None,
    }
}
