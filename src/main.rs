use actix::{Actor, Addr, SyncArbiter};
use cardgame::ai_player::AiPlayer;
use cardgame::game_logic::run_game;
use cardgame::human_player::HumanPlayer;
use cardgame::scryer_actor::{create_scryer_actor, ScryerActor};

#[actix::main]
async fn main() {
    let scryer_game: Addr<ScryerActor> = SyncArbiter::start(1, create_scryer_actor);
    let scryer_ai: Addr<ScryerActor> = SyncArbiter::start(6, create_scryer_actor);
    let player1 = AiPlayer {scryer: scryer_ai}.start();
    let player2 = HumanPlayer {}.start();
    let final_state = run_game(scryer_game, player1, player2, None, None).await;
    println!("{:?}", final_state);
}
