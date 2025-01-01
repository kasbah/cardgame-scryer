use actix::{Actor, Addr, SyncArbiter};
use cardgame::ai_player::AiPlayer;
use cardgame::game_logic::run_game;
use cardgame::human_player::HumanPlayer;
use cardgame::scryer_actor::ScryerActor;
use scryer_prolog::MachineBuilder;

#[actix::main]
async fn main() {
    let scryer: Addr<ScryerActor> = SyncArbiter::start(1, || {
        let mut machine = MachineBuilder::default().build();
        let file_content = include_str!("logic.pl");
        machine.load_module_string("logic", file_content);
        ScryerActor { scryer: machine }
    });
    let player1 = AiPlayer {}.start();
    let player2 = HumanPlayer {}.start();
    let final_state = run_game(scryer, player1, player2, None, None).await;
    println!("{:?}", final_state);
}
