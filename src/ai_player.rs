use crate::dummy_player::DummyPlayer;
use crate::game_logic::{run_game, GameState};
use crate::move_request::{MoveChoice, MoveRequest};
use crate::scryer_actor::{Query, ScryerActor};
use crate::{random::random_choice, scryer_types::to_prolog};
use actix::{Actor, Addr, AtomicResponse, Context, Handler, WrapFuture};
use futures::future::join_all;
use scryer_prolog::{LeafAnswer, Term};
use std::collections::BTreeMap;

pub fn create_ai_player(scryer: Addr<ScryerActor>, player: &'static str) -> AiPlayer {
    let dummy_player = DummyPlayer {}.start();
    AiPlayer {
        player,
        scryer,
        dummy_player,
    }
}

pub struct AiPlayer {
    pub player: &'static str,
    pub scryer: Addr<ScryerActor>,
    pub dummy_player: Addr<DummyPlayer>,
}

impl Actor for AiPlayer {
    type Context = Context<Self>;
}

impl Handler<MoveRequest> for AiPlayer {
    type Result = AtomicResponse<Self, MoveChoice>;

    fn handle(&mut self, req: MoveRequest, _ctx: &mut Context<Self>) -> Self::Result {
        let scryer = self.scryer.clone();
        let dummy_player = self.dummy_player.clone();
        let player = self.player;

        AtomicResponse::new(
            Box::pin(
                async move {
                    make_ai_move(player, &scryer, &dummy_player, req.visible_state, req.options).await
                }
                .into_actor(self),
            ),
        )
    }
}

async fn make_ai_move(
    player: &str,
    scryer: &Addr<ScryerActor>,
    dummy_player: &Addr<DummyPlayer>,
    visible_state: GameState,
    options: Vec<GameState>,
) -> usize {
    let scores = join_all(options.into_iter().map(move |option| {
        let visible_state = visible_state.clone();
        async move {
            let mut score: i128 = 0;
            for _ in 0..100 {
                let mut possible_state = get_possible_state(scryer, &visible_state).await;
                possible_state.extend(option.clone());
                let next_state = run_game(
                    scryer,
                    dummy_player,
                    dummy_player,
                    Some(possible_state),
                    Some(1),
                )
                .await;
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

                if player == "player1" {
                    score += player1_n_cards as i128 - player2_n_cards as i128;
                } else {
                    score += player2_n_cards as i128 - player1_n_cards as i128;
                }
            }
            score
        }
    }))
    .await;
    let max = scores.iter().max().unwrap();
    scores.iter().position(|x| x == max).unwrap()
}

async fn get_possible_state(
    scryer: &Addr<ScryerActor>,
    visible_state: &BTreeMap<String, Term>,
) -> BTreeMap<String, Term> {
    let mut possible_state = BTreeMap::new();
    let mut used_facts: BTreeMap<String, Vec<Term>> = BTreeMap::new();

    for (_k, v) in visible_state.iter() {
        get_concrete_facts(v, &mut used_facts);
    }

    for (k, v) in visible_state.iter() {
        possible_state.insert(
            k.clone(),
            get_possible_facts(scryer, v, &mut used_facts).await,
        );
    }

    possible_state
}

fn get_concrete_facts(v: &Term, used_facts: &mut BTreeMap<String, Vec<Term>>) {
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

async fn get_possible_facts(
    scryer: &Addr<ScryerActor>,
    v: &Term,
    used_facts: &mut BTreeMap<String, Vec<Term>>,
) -> Term {
    match v {
        Term::List(list) => {
            let mut result = Vec::new();
            for x in list {
                result.push(Box::pin(get_possible_facts(scryer, x, used_facts)).await);
            }
            Term::List(result)
        }
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
                .send(Query(query))
                .await
                .expect("No response in get_possible_facts")
                .into_iter()
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

fn has_prolog_vars(v: &Term) -> bool {
    match v {
        Term::Compound(_, args) => args.iter().any(|x| match x {
            Term::Var(_) => true,
            _ => false,
        }),
        _ => false,
    }
}
