use cardgame::game_logic;
use cardgame::random;

pub fn make_random_move(_: game_logic::GameState, options: &Vec<game_logic::GameState>) -> usize {
    random::random_choice(options)
}

fn main() {
    let final_state = game_logic::run_game(&make_random_move, &make_random_move, None, Some(100));
    println!("{:?}", final_state);
}
