use crate::game_logic::{run_game, GameState};
use crate::{random::random_choice, scryer_types::to_prolog};
use scryer_prolog::{LeafAnswer, Machine as ScryerMachine, Term};
use std::collections::BTreeMap;

pub fn create_ai() -> impl Fn(&GameState, &Vec<GameState>) -> usize {
    fn make_ai_move(_: &GameState, options: &Vec<GameState>) -> usize {
        0
    }
    make_ai_move
}

pub fn get_possible_state(
    scryer: &mut ScryerMachine,
    visible_state: &BTreeMap<String, Term>,
) -> BTreeMap<String, Term> {
    let mut possible_state = BTreeMap::new();
    let mut used_facts: BTreeMap<String, Vec<Term>> = BTreeMap::new();

    for (_k, v) in visible_state.iter() {
        get_concrete_facts(v, &mut used_facts);
    }

    for (k, v) in visible_state.iter() {
        possible_state.insert(k.clone(), get_possible_facts(scryer, v, &mut used_facts));
    }

    possible_state
}

pub fn get_concrete_facts(v: &Term, used_facts: &mut BTreeMap<String, Vec<Term>>) {
    match v {
        Term::List(list) => {
            for x in list {
                get_concrete_facts(x, used_facts);
            }
        }
        Term::Compound(name, _) if !has_prolog_vars(v) => {
            let used = used_facts.entry(name.into()).or_insert_with(Vec::new);
            used.push(v.clone());
        }
        _ => {}
    }
}

pub fn get_possible_facts(
    scryer: &mut ScryerMachine,
    v: &Term,
    used_facts: &mut BTreeMap<String, Vec<Term>>,
) -> Term {
    match v {
        Term::List(list) => Term::List(
            list.iter()
                .map(|x| get_possible_facts(scryer, x, used_facts))
                .collect(),
        ),
        Term::Compound(name, _) if has_prolog_vars(v) => {
            let used = used_facts.entry(name.into()).or_insert_with(Vec::new);
            let unique_var = "X_fa1d383e9ex1_kaspar_cardgame";

            let query = format!(
                r#"{}, {unique_var} = {}, \+ member({unique_var}, {})."#,
                to_prolog(v),
                to_prolog(v),
                to_prolog(&Term::List(used.clone()))
            );

            let answers = scryer
                .run_query(&query)
                .filter(|answer| match answer {
                    Ok(LeafAnswer::LeafAnswer { bindings, .. }) => {
                        bindings.contains_key(unique_var)
                    }
                    _ => false,
                })
                .collect::<Vec<_>>();

            let choice = random_choice(&answers);
            let answer = answers[choice].clone();
            if let Ok(LeafAnswer::LeafAnswer { bindings, .. }) = answer {
                let x = bindings.get(unique_var).unwrap();
                used.push(x.clone());
                x.clone()
            } else {
                unreachable!("Unexpected answer: {:?}", answer);
            }
        }
        _ => v.clone(),
    }
}

pub fn has_prolog_vars(v: &Term) -> bool {
    match v {
        Term::Compound(_, args) => args.iter().any(|x| match x {
            Term::Var(_) => true,
            _ => false,
        }),
        _ => false,
    }
}
