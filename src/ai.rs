//use scryer_prolog::{Machine as ScryerMachine, Term};
//use std::collections::BTreeMap;

//pub fn get_possible_state(
//    scryer: &mut ScryerMachine,
//    visible_state: &BTreeMap<String, Term>,
//) -> BTreeMap<String, Term> {
//    let mut possible_state = BTreeMap::new();
//    let mut used_facts: BTreeMap<String, Vec<Term>> = BTreeMap::new();
//
//    for (_k, v) in visible_state.iter() {
//        get_concrete_facts(v, &mut used_facts);
//    }
//
//    for (k, v) in visible_state.iter() {
//        possible_state.insert(k.clone(), get_possible_facts(scryer, v, &mut used_facts));
//    }
//
//    possible_state
//}
//
//pub fn has_prolog_vars(v: &Term) -> bool {
//    match v {
//        Term::Compound(_, args) => args.iter().any(|x| match x {
//            Term::Var(_) => true,
//            _ => false,
//        }),
//        _ => false,
//    }
//}
//
//pub fn get_concrete_facts(v: &Term, used_facts: &mut BTreeMap<String, Vec<Term>>) {
//    match v {
//        Term::List(list) => {
//            for x in list {
//                get_concrete_facts(x, used_facts);
//            }
//        }
//        Term::Compound(name, _) if !has_prolog_vars(v) => {
//            let used = used_facts.entry(name.clone()).or_insert(vec![]);
//            used.push(v.clone());
//        }
//        _ => {}
//    }
//}
