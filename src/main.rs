use cardgame::game_logic::{run_game, GameState};
use cardgame::random::random_choice;

#[allow(clippy::ptr_arg)]
pub fn make_random_move(_: &GameState, options: &Vec<GameState>) -> usize {
    random_choice(options)
}

fn main() {
    let final_state = run_game(make_random_move, make_random_move, None, Some(100));
    println!("{:?}", final_state);
}
