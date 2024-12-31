use crate::game_logic::{run_game, GameState};
use crate::{random::random_choice, scryer_types::to_prolog};
use scryer_prolog::{LeafAnswer, Machine as ScryerMachine, MachineBuilder, Term};
use std::collections::BTreeMap;

pub fn create_ai() -> impl FnMut(&GameState, &Vec<GameState>) -> usize {
    let mut scryer = MachineBuilder::default().build();
    let file_content = include_str!("logic.pl");
    scryer.load_module_string("logic", file_content);


    let make_ai_move = move |visible_state: &GameState, options: &Vec<GameState>| {
        let mut scores = Vec::new();
        for option in options {
            let mut score: i128 = 0;
            for _ in 0..100 {
                let mut possible_state = get_possible_state(&mut scryer, visible_state);
                possible_state.extend(option.clone());
                let next_state = run_game(
                    &mut scryer,
                    &mut |_, __| 0,
                    &mut |_, __| 0,
                    Some(possible_state),
                    Some(1),
                );
                let deck1 = next_state.get("deck1").unwrap();
                let deck2 = next_state.get("deck2").unwrap();
                let win_pile1 = next_state.get("win_pile1").unwrap();
                let win_pile2 = next_state.get("win_pile2").unwrap();
                
                let player1_n_cards = match (deck1, win_pile1) {
                    (Term::List(d1), Term::List(w1)) => d1.len() + w1.len(),
                    _ => 0,
                };
                
                let player2_n_cards = match (deck2, win_pile2) {
                    (Term::List(d2), Term::List(w2)) => d2.len() + w2.len(),
                    _ => 0,
                };
                score += player1_n_cards as i128 - player2_n_cards as i128;
            }
            scores.push(score);
        }
        let max = scores.iter().max().unwrap();
        scores.iter().position(|x| x == max).unwrap()
    };
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
