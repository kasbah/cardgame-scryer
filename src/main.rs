use actix::Actor;
use cardgame::ai_player::AiPlayer;
use cardgame::game_logic::run_game;
use cardgame::human_player::HumanPlayer;
use scryer_prolog::MachineBuilder;

#[actix::main]
async fn main() {
    let mut scryer = MachineBuilder::default().build();
    let file_content = include_str!("logic.pl");
    scryer.load_module_string("logic", file_content);
    let player1 = AiPlayer {}.start();
    let player2 = HumanPlayer {}.start();
    let final_state = run_game(&mut scryer, player1, player2, None, None).await;
    println!("{:?}", final_state);
}
