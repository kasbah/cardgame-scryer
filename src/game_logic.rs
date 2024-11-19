use crate::prolog_types::{from_prolog_assoc, to_prolog_assoc};
use crate::random::random_choice;
use scryer_prolog::{LeafAnswer, Machine, MachineBuilder, Term};
use std::collections::BTreeMap;

pub type GameState = BTreeMap<String, Term>;

pub fn run_game(
    resolve_player1: impl Fn(GameState, &Vec<GameState>) -> usize,
    resolve_player2: impl Fn(GameState, &Vec<GameState>) -> usize,
    initial_state: Option<GameState>,
    max_steps: Option<usize>,
) -> GameState {
    let mut machine = MachineBuilder::default().build();
    let file_content = include_str!("logic.pl");
    machine.load_module_string("logic", file_content);

    run_game_with_machine(
        &mut machine,
        resolve_player1,
        resolve_player2,
        initial_state,
        max_steps,
    )
}

fn get_initial_state(machine: &mut Machine) -> GameState {
    let query = r#"init(State)."#;
    let answer = query_once_binding(machine, query, "State");
    match answer {
        Some(term) => from_prolog_assoc(&term),
        None => panic!("Could not get initial state"),
    }
}

fn resolve_randomness(machine: &mut Machine, state: GameState) -> GameState {
    let state_in = to_prolog_assoc(&state, "StateIn");
    let query = format!(r#"{state_in}, random_options(StateIn, StateOut)."#);

    let answers: Vec<Term> = machine
        .run_query(&query)
        .filter_map(|answer| match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => {
                bindings.get("StateOut").map(|x| x.to_owned())
            }
            _ => None,
        })
        .collect();

    if answers.is_empty() {
        return state;
    }

    let i = random_choice(&answers);
    from_prolog_assoc(&answers[i])
}

fn resolve_next(machine: &mut Machine, state: GameState) -> GameState {
    let state_in = to_prolog_assoc(&state, "StateIn");
    let query = format!(r#"{state_in}, once(next(StateIn, StateOut))."#);
    let answer = query_once_binding(machine, &query, "StateOut");
    answer.map(|term| from_prolog_assoc(&term)).unwrap_or(state)
}

fn get_visible(machine: &mut Machine, state: &GameState, player: &str) -> GameState {
    let state_in = to_prolog_assoc(state, "StateIn");
    let query = format!(r#"{state_in}, once(sees({player}, StateIn, VisibleState))."#);
    let answer = query_once_binding(machine, &query, "VisibleState");
    match answer {
        Some(term) => from_prolog_assoc(&term),
        None => BTreeMap::new(),
    }
}

fn get_player_options(machine: &mut Machine, state: &GameState, player: &str) -> Vec<GameState> {
    let state_in = to_prolog_assoc(state, "StateIn");
    let query = format!(r#"{state_in}, player_options({player}, StateIn, PartialStateOut)."#,);

    machine
        .run_query(&query)
        .filter_map(|answer| match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => bindings
                .get("PartialStateOut")
                .map(|term| from_prolog_assoc(term)),
            _ => None,
        })
        .collect()
}

pub fn run_game_with_machine(
    machine: &mut Machine,
    resolve_player1: impl Fn(GameState, &Vec<GameState>) -> usize,
    resolve_player2: impl Fn(GameState, &Vec<GameState>) -> usize,
    initial_state: Option<GameState>,
    max_steps: Option<usize>,
) -> GameState {
    let mut state = match initial_state {
        Some(s) => s,
        None => get_initial_state(machine),
    };

    let finished = Term::Atom("finished".to_string());

    let mut steps = 0;
    while state
        .get("game_phase")
        .map(|t| t != &finished)
        .expect("Missing game_phase")
        && steps <= max_steps.unwrap_or(usize::MAX)
    {
        state = resolve_randomness(machine, state);

        state = resolve_next(machine, state);

        let player1_visible = get_visible(machine, &state, "player1");
        let player1_options = get_player_options(machine, &state, "player1");
        if !player1_options.is_empty() {
            let player1_choice = resolve_player1(player1_visible, &player1_options);
            state.extend(player1_options[player1_choice].clone());
        }

        let player2_visible = get_visible(machine, &state, "player2");
        let player2_options = get_player_options(machine, &state, "player2");
        if !player2_options.is_empty() {
            let player2_choice = resolve_player2(player2_visible, &player2_options);
            state.extend(player2_options[player2_choice].clone());
        }

        steps += 1;
    }
    state
}

fn query_once_binding(machine: &mut Machine, query: &str, var: &str) -> Option<Term> {
    let mut answers = machine.run_query(query);
    let answer = answers.next();
    match answer {
        Some(Ok(LeafAnswer::LeafAnswer { bindings, .. })) => match bindings.get(var) {
            Some(x) => Some(x.to_owned()),
            None => None,
        },
        _ => None,
    }
}
