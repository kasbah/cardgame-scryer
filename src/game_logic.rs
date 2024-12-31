use crate::move_messages::MoveOptions;
use crate::random::random_choice;
use crate::scryer_types::{from_prolog_assoc, to_prolog_assoc};
use crate::scryer_util::query_once_binding;
use actix::dev::ToEnvelope;
use actix::{Actor, Addr, Handler};
use scryer_prolog::{LeafAnswer, Machine as ScryerMachine, Term};
use std::collections::BTreeMap;

pub type GameState = BTreeMap<String, Term>;

pub async fn run_game<Player1: Handler<MoveOptions>, Player2: Handler<MoveOptions>>(
    scryer: &mut ScryerMachine,
    player1: Addr<Player1>,
    player2: Addr<Player2>,
    initial_state: Option<GameState>,
    max_steps: Option<usize>,
) -> GameState
where
    <Player1 as Actor>::Context: ToEnvelope<Player1, MoveOptions>,
    <Player2 as Actor>::Context: ToEnvelope<Player2, MoveOptions>,
{
    let mut state = match initial_state {
        Some(s) => s,
        None => get_initial_state(scryer),
    };

    let finished = Term::Atom("finished".to_string());

    let mut steps = 0;
    while state
        .get("game_phase")
        .map(|t| t != &finished)
        .expect("Missing game_phase")
        && steps <= max_steps.unwrap_or(usize::MAX)
    {
        state = resolve_randomness(scryer, state);

        state = resolve_next(scryer, state);

        let player1_visible = get_visible(scryer, &state, "player1");
        let mut player1_options = get_player_options(scryer, &state, "player1");
        if !player1_options.is_empty() {
            let player1_choice = player1
                .send(MoveOptions {
                    current: player1_visible,
                    options: player1_options.clone(),
                })
                .await
                .expect("No response from player1");
            state.append(&mut player1_options[player1_choice]);
        }

        let player2_visible = get_visible(scryer, &state, "player2");
        let mut player2_options = get_player_options(scryer, &state, "player2");
        if player2_options.is_empty() {
            player2_options.push(BTreeMap::new());
        }
        let player2_choice = player2
            .send(MoveOptions {
                current: player2_visible,
                options: player2_options.clone(),
            })
            .await
            .expect("No response from player2");
        state.append(&mut player2_options[player2_choice]);

        steps += 1;
    }
    state
}

fn get_initial_state(scryer: &mut ScryerMachine) -> GameState {
    let query = r#"init(State)."#;
    let answer = query_once_binding(scryer, query, "State");
    match answer {
        Some(term) => from_prolog_assoc(&term),
        None => panic!("Could not get initial state"),
    }
}

fn resolve_randomness(scryer: &mut ScryerMachine, state: GameState) -> GameState {
    let state_in = to_prolog_assoc(&state, "StateIn");
    let query = format!(r#"{state_in}, random_options(StateIn, StateOut)."#);

    let answers: Vec<Term> = scryer
        .run_query(&query)
        .filter_map(|answer| match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => bindings.get("StateOut").cloned(),
            _ => None,
        })
        .collect();

    if answers.is_empty() {
        return state;
    }

    let i = random_choice(&answers);
    from_prolog_assoc(&answers[i])
}

fn resolve_next(scryer: &mut ScryerMachine, state: GameState) -> GameState {
    let state_in = to_prolog_assoc(&state, "StateIn");
    let query = format!(r#"{state_in}, once(next(StateIn, StateOut))."#);
    let answer = query_once_binding(scryer, &query, "StateOut");
    let x = answer.map(|term| from_prolog_assoc(&term));
    x.unwrap_or(state)
}

fn get_visible(scryer: &mut ScryerMachine, state: &GameState, player: &str) -> GameState {
    let state_in = to_prolog_assoc(state, "StateIn");
    let query = format!(r#"{state_in}, once(sees({player}, StateIn, VisibleState))."#);
    let answer = query_once_binding(scryer, &query, "VisibleState");
    match answer {
        Some(term) => from_prolog_assoc(&term),
        None => BTreeMap::new(),
    }
}

fn get_player_options(
    scryer: &mut ScryerMachine,
    state: &GameState,
    player: &str,
) -> Vec<GameState> {
    let state_in = to_prolog_assoc(state, "StateIn");
    let query = format!(r#"{state_in}, player_options({player}, StateIn, PartialStateOut)."#,);

    scryer
        .run_query(&query)
        .filter_map(|answer| match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => {
                bindings.get("PartialStateOut").map(from_prolog_assoc)
            }
            _ => None,
        })
        .collect()
}
