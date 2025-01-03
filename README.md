# A Card Game Using Scryer Prolog

A card game prototype using [Scryer Prolog](https://scryer.pl) as a library in Rust.

- It's a [Top Trumps](https://en.wikipedia.org/wiki/Top_Trumps) style game using planets and exo-planets
- The rules are defined in Prolog and very inspired by the [Game Description Language](https://en.wikipedia.org/wiki/Game_Description_Language). The relevant source file is [logic.pl](https://github.com/kasbah/cardgame-scryer/blob/main/src/logic.pl)
  - The cards are the only defined facts
  - There's an `init` rule that gives us the initial game state
  - There's a `random_options` rule that gives us things to randomize based on the game state
  - There's a `next` rule to advance the game state 
  - There's a `sees` rule that tells us what each player can see
  - There's a `player_options` rule that gives a player the possible moves based on the game state
- This is all pulled together in Rust, in the main game loop: [game_logic.rs](https://github.com/kasbah/cardgame-scryer/blob/main/src/game_logic.rs#L32-L70). It queries these rules in order in a loop and modifies the game state. The idea being you could change the logic of the game in Prolog and it would continue to work without modifying the Rust code.
- It has a rudimentary inefficient AI that does a Monte-Carlo-esque thing (but backwards in time instead of forwards? :thinking:), again, the idea being to develop an AI that would continue to work if the game rules are changed
- I did wrap Scryer in an Actix actor so the AI, for instance, can query 6 Scryer machines running on separate threads. 


## Running

You can clone the repo and run the game. It will print some game state and then the AI will start thinking about which category to pick. Once it figures it out you will be shown which cards were drawn. If the AI ever loses it's your turn to pick the category.

```
git clone https://github.com/kasbah/cardgame-scryer/
cd cardgame-scryer
cargo run --release 
```

<details>

<summary>example output</summary>

```
   Compiling cardgame v0.1.0 (/home/kaspar/projects/blank-planet/scryer/cardgame)
    Finished `release` profile [optimized] target(s) in 5.12s
     Running `target/release/cardgame`
Game phase: dealing
Player turn: none
Your cards: 0
Opponent cards: 1
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 1
Opponent cards: 1
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 1
Opponent cards: 2
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 2
Opponent cards: 2
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 2
Opponent cards: 3
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 3
Opponent cards: 3
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 3
Opponent cards: 4
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 4
Opponent cards: 4
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 4
Opponent cards: 5
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 5
Opponent cards: 5
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 5
Opponent cards: 6
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 6
Opponent cards: 6
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 6
Opponent cards: 7
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 7
Opponent cards: 7
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 7
Opponent cards: 8
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 8
Opponent cards: 8
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 8
Opponent cards: 9
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 9
Opponent cards: 9
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 9
Opponent cards: 10
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 10
Opponent cards: 10
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 10
Opponent cards: 11
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 11
Opponent cards: 11
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 11
Opponent cards: 12
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 12
Opponent cards: 12
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 12
Opponent cards: 13
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 13
Opponent cards: 13
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 13
Opponent cards: 14
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 14
Opponent cards: 14
------------------------------------
Game phase: dealing
Player turn: none
Your cards: 14
Opponent cards: 15
------------------------------------
Game phase: playing
Player turn: player1
Your cards: 15
Opponent cards: 15
------------------------------------
Game phase: scoring
Player turn: player1
Your cards: 14
Opponent cards: 14
Opponent picked: Compound("category", [Integer(0), Atom("distance")])
------------------------------------
Your card:
Name:             c_hat_p_7_b
Distance:         1000 light-years
Temperature:      2457 °C
Orbit time:       22 days
Radius:           1639.999 × Earth
Mass:             57450 × Earth
Earth similarity: 0.03
--------------
Opponent card:
Name:             c_kepler_10_c
Distance:         605 light-years
Temperature:      311 °C
Orbit time:       453 days
Radius:           235.5 × Earth
Mass:             1140 × Earth
Earth similarity: 0.47
--------------
option 0: {}
Enter option: 
Game phase: evaluating
Player turn: player2
Your cards: 16
Opponent cards: 14
------------------------------------
Game phase: playing
Player turn: player2
Your cards: 16
Opponent cards: 14
------------------------------------
Your card:
Name:             c_mercury
Distance:         0.0000097 light-years
Temperature:      350 °C
Orbit time:       880 days
Radius:           38.3 × Earth
Mass:             5.5 × Earth
Earth similarity: 0.43
--------------
option 0: {"selected_category": Compound("category", [Integer(0), Atom("distance")])}
option 1: {"selected_category": Compound("category", [Integer(1), Atom("temp")])}
option 2: {"selected_category": Compound("category", [Integer(2), Atom("orbit_time")])}
option 3: {"selected_category": Compound("category", [Integer(3), Atom("radius")])}
option 4: {"selected_category": Compound("category", [Integer(4), Atom("mass")])}
option 5: {"selected_category": Compound("category", [Integer(5), Atom("earth_similarity")])}
Enter option: 

```

</details>
