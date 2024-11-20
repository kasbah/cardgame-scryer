use cardgame::ai::create_ai;
use cardgame::game_logic::{run_game, GameState};
use cardgame::random::random_choice;
use scryer_prolog::Term;
use std::io::{self, Write};

#[allow(clippy::ptr_arg)]
pub fn make_random_move(_: &GameState, options: &Vec<GameState>) -> usize {
    random_choice(options)
}

fn show_card(card: &Term) {
    if let Term::Compound(_, args) = card {
        assert!(args.len() >= 7, "Card should have at least 7 arguments");

        let get_num = |term: &Term| -> f64 {
            if let Term::Integer(n) = term {
                n.to_f64().unwrap()
            } else {
                0.0
            }
        };

        let get_str = |term: &Term| -> String {
            if let Term::Atom(s) = term {
                s.clone()
            } else {
                String::from("unknown")
            }
        };

        println!("Name:             {}", get_str(&args[0]));
        println!(
            "Distance:         {} light-years",
            get_num(&args[1]) / 10000000.0
        );
        println!("Temperature:      {} °C", get_num(&args[2]));
        println!("Orbit time:       {} days", get_num(&args[3]) / 10.0);
        println!("Radius:           {} × Earth", get_num(&args[4]) / 1000.0);
        println!("Mass:             {} × Earth", get_num(&args[5]) / 1000.0);
        println!("Earth similarity: {}", get_num(&args[6]) / 100.0);
        println!("--------------");
    }
}

fn show_state(state: &GameState) {
    let get_term = |key: &str| -> Option<&Term> { state.get(key) };

    let get_list_len = |key: &str| -> usize {
        if let Some(Term::List(list)) = get_term(key) {
            list.len()
        } else {
            0
        }
    };

    println!(
        "Game phase: {}",
        state
            .get("game_phase")
            .map(|term| {
                match term {
                    Term::Atom(s) => s,
                    _ => "unknown",
                }
            })
            .unwrap_or("unknown")
    );
    println!(
        "Player turn: {}",
        get_term("player_turn")
            .map(|term| {
                match term {
                    Term::Atom(s) => s,
                    _ => "unknown",
                }
            })
            .unwrap_or("unknown")
    );
    println!(
        "Your cards: {}",
        get_list_len("deck2") + get_list_len("win_pile2")
    );
    println!(
        "Opponent cards: {}",
        get_list_len("deck1") + get_list_len("win_pile1")
    );

    if let (Some(Term::Atom(phase)), Some(Term::Atom(turn))) =
        (get_term("game_phase"), get_term("player_turn"))
    {
        if phase == "scoring" && turn == "player1" {
            println!(
                "Opponent picked: {:?}",
                get_term("selected_category").unwrap_or(&Term::Atom("unknown".to_string()))
            );
        }
    }
    println!("------------------------------------");

    if let (Some(Term::Atom(phase)), Some(Term::Atom(turn))) =
        (get_term("game_phase"), get_term("player_turn"))
    {
        if phase == "playing" && turn == "player2" {
            if let Some(Term::List(deck)) = state.get("deck2") {
                if let Some(card) = deck.first() {
                    println!("Your card:");
                    show_card(card);
                }
            }
        } else if phase == "scoring" {
            if let Some(Term::List(table_cards)) = get_term("cards_on_table") {
                if let Some(card) = table_cards.get(1) {
                    println!("Your card:");
                    show_card(card);
                }
                if let Some(card) = table_cards.get(0) {
                    println!("Opponent card:");
                    show_card(card);
                }
            }
        }
    }
}

pub fn resolve_human_player(state: &GameState, options: &Vec<GameState>) -> usize {
    show_state(state);

    let mut n = 0;
    match (state.get("game_phase"), state.get("player_turn")) {
        (Some(Term::Atom(phase)), Some(Term::Atom(player_turn)))
            if (phase == "playing" && player_turn == "player2") || (phase == "scoring") =>
        {
            for (i, option) in options.iter().enumerate() {
                println!("option {}: {:?}", i, option);
            }

            print!("Enter option: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap_or(0);

            if let Ok(num) = input.trim().parse::<usize>() {
                if num < options.len() {
                    n = num;
                }
            }
        }
        _ => n = 0,
    };

    n
}

fn main() {
    let mut make_ai_move = create_ai();
    let final_state = run_game(&mut make_ai_move, &mut resolve_human_player, None, None);
    println!("{:?}", final_state);
}
