:- use_module(library(assoc)).
:- use_module(library(clpz)).
:- use_module(library(lists)).

put_assoc_k_v(Key-Value, Assoc, AssocOut) :-
  put_assoc(Key, Assoc, Value, AssocOut).

update_assoc(List, Assoc, AssocOut) :-
  foldl(put_assoc_k_v, List, Assoc, AssocOut).


role(player1).
role(player2).


% card(Id,                   Distance,    Temp, OrbitTime,    Radius,   Mass,      EarthSimilarity)
card(c_mercury,              97,           350,  8800,        38300,    5500,      43).
card(c_venus,                44,           480,  22470,       94900,    81500,     43).
card(c_earth,                0,            14,   36530,       100000,   100000,    99).
card(c_mars,                 83,           -23,  68800,       53200,    10700,     75).
card(c_jupiter,              666,          -150, 433280,      1120900,  31780000,  23).
card(c_saturn,               1343,         -210, 1075570,     944900,   9520000,   10).
card(c_uranus,               2888,         -210, 3068710,     400699,   1450000,   15).
card(c_neptune,              4593,         -220, 6019000,     388300,   1710000,   12).
card(c_proxima_centauri_b,   42400000,     -39,  1120,        94000,    107000,    82).
card(c_teegardens_star_b,    130000000,    3,    490,         105000,   115999,    96).
card(c_gliese_436_b,         320000000,    439,  2643,        432700,   2136000,   29).
card(c_gliese_12_b,          400000000,    42,   1280,        95800,    387000,    66).
card(c_55_cancri_e,          410000000,    3498, 74,          187500,   799000,    5).
card(c_gliese_504_b,         570000000,    271,  48618000,    1052000,  127100000, 33).
card(c_hd_189733_b,          650000000,    919,  220,         1251560,  35700000,  12).
card(c_toi_700_d,            1020000000,   -4,   3740,        107300,   125000,    93).
card(c_hr_5183_b,            1030000000,   -102, 3725550,     1017590,  102690000, 38).
card(c_2mass_j2126_8140,     1114000000,   1390, 32872500000, 15200000, 422800000, 1).
card(c_k2_18_b,              1240000000,   -8,   3290,        237000,   892000,    77).
card(c_kepler_16_b,          2450000000,   -20,  22880,       826460,   10587000,  46).
card(c_toi_3757_b,           5910000000,   486,  340,         1200000,  8500000,   15).
card(c_kepler_22_b,          6000000000,   6,    28990,       213499,   910000,    84).
card(c_kepler_10_c,          6050000000,   311,  4530,        235500,   1140000,   47).
card(c_kelt_9_b,             6670000000,   3776, 150,         2079700,  91600000,  2).
card(c_tres_2_b,             7030000000,   1612, 250,         1351000,  38100000,  6).
card(c_hat_p_7_b,            10000000000,  2457, 220,         1639999,  57450000,  3).
card(c_kepler_452_b,         14000000000,  -8,   38484,       150000,   500000,    81).
card(c_psr_j1719_1438_b,     40000000000,  1000, 9,           400000,   33000000,  12).
card(c_psr_b1620_26_b,       58710000000,  -201, 2483700,     1293400,  79500000,  13).
card(c_ogle_2005_blg_390l_b, 220000000000, -220, 328725,      221000,   550000,    15).


all_cards(Cards) :-
  setof(card(Id, Distance, Temp, OrbitTime, Radius, Mass, EarthSimilarity),
        card(Id, Distance, Temp, OrbitTime, Radius, Mass, EarthSimilarity),
        Cards).



a_card(Card) :-
  all_cards(Cards),
  member(Card, Cards).



init(GameState) :-
  list_to_assoc([
    deck1-[],
    deck2-[],
    win_pile1-[],
    win_pile2-[],
    cards_on_table-[],
    game_phase-dealing,
    player_turn-none,
    selected_category-none
  ], GameState).



random_options(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = dealing,
  get_assoc(deck1, GameStateIn, Deck1),
  get_assoc(deck2, GameStateIn, Deck2),
  a_card(Card),
  \+ member(Card, Deck1),
  \+ member(Card, Deck2),
  length(Deck1, Deck1Length),
  length(Deck2, Deck2Length),
  (Deck1Length #= Deck2Length
    -> put_assoc(deck1, GameStateIn, [Card | Deck1], GameStateOut)
    ;  put_assoc(deck2, GameStateIn, [Card | Deck2], GameStateOut)
  ).


random_options(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = restocking_player1,
  get_assoc(win_pile1, GameStateIn, WinPile1),
  \+ length(WinPile1, 0),
  get_assoc(deck1, GameStateIn, Deck1),
  member(Card, WinPile1),
  select(Card, WinPile1, RestOfWinPile1),
  put_assoc(deck1, GameStateIn, [Card | Deck1], GameStateOut0),
  put_assoc(win_pile1, GameStateOut0, RestOfWinPile1, GameStateOut).


random_options(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = restocking_player2,
  get_assoc(win_pile2, GameStateIn, WinPile2),
  \+ length(WinPile2, 0),
  get_assoc(deck2, GameStateIn, Deck2),
  member(Card, WinPile2),
  select(Card, WinPile2, RestOfWinPile2),
  update_assoc([
    deck2-[Card | Deck2],
    win_pile2-[RestOfWinPile2]
  ], GameStateIn, GameStateOut).



next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = dealing,
  \+ random_options(GameStateIn, GameStateOut),
  put_assoc(game_phase, GameStateIn, playing, GameStateOut0),
  put_assoc(player_turn, GameStateOut0, player1, GameStateOut).


next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = playing,
  get_assoc(deck1, GameStateIn, Deck1),
  get_assoc(deck2, GameStateIn, Deck2),
  Deck1 = [Player1Card | RestOfDeck1 ],
  Deck2 = [Player2Card | RestOfDeck2 ],
  update_assoc([
    cards_on_table-[Player1Card, Player2Card],
    game_phase-scoring,
    deck1-RestOfDeck1,
    deck2-RestOfDeck2
  ], GameStateIn, GameStateOut).


next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = scoring,
  get_assoc(cards_on_table, GameStateIn, CardsOnTable),
  CardsOnTable = [Player1Card, Player2Card],
  write_term(Player1Card, []),
  write_term(Player2Card, []),
  get_assoc(selected_category, GameStateIn, SelectedCategory),
  category(N, _) = SelectedCategory,
  Player1Card =.. Player1CardDeconstructed,
  Player2Card =.. Player2CardDeconstructed,
  M #= N + 2,
  nth0(M, Player1CardDeconstructed, Player1CardValue),
  nth0(M, Player2CardDeconstructed, Player2CardValue),
  update_assoc([
    cards_on_table-none,
    game_phase-evaluating
  ], GameStateIn, GameStateOut0),
  (Player1CardValue #> Player2CardValue
    -> (get_assoc(win_pile1, GameStateIn, WinPile1),
        NewWinPile1 = [Player1Card, Player2Card | WinPile1],
        update_assoc([
          win_pile1-NewWinPile1,
          player_turn-player1
        ], GameStateOut0, GameStateOut))
    ;  (get_assoc(win_pile2, GameStateIn, WinPile2),
        NewWinPile2 = [Player1Card, Player2Card | WinPile2],
        update_assoc([
          win_pile2-NewWinPile2,
          player_turn-player2
        ], GameStateOut0, GameStateOut))
  ).


next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = evaluating,
  get_assoc(deck1, GameStateIn, Deck1),
  length(Deck1, 0),
  get_assoc(win_pile1, GameStateIn, WinPile1),
  length(WinPile1, 0),
  update_assoc([
    game_phase-finished,
    winner-player2
  ], GameStateIn, GameStateOut).



next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = evaluating,
  get_assoc(deck2, GameStateIn, Deck2),
  length(Deck2, 0),
  get_assoc(win_pile2, GameStateIn, WinPile2),
  length(WinPile2, 0),
  update_assoc([
    game_phase-finished,
    winner-player1
  ], GameStateIn, GameStateOut).


next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = evaluating,
  get_assoc(deck1, GameStateIn, Deck1),
  length(Deck1, 0),
  get_assoc(win_pile1, GameStateIn, WinPile1),
  length(WinPile1, WinPile1Length),
  WinPile1Length #> 0,
  put_assoc(game_phase, GameStateIn, restocking_player1, GameStateOut).


next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = evaluating,
  get_assoc(deck2, GameStateIn, Deck2),
  length(Deck2, 0),
  get_assoc(win_pile2, GameStateIn, WinPile2),
  length(WinPile2, WinPile2Length),
  WinPile2Length #> 0,
  put_assoc(game_phase, GameStateIn, restocking_player2, GameStateOut).


next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = restocking_player1,
  get_assoc(win_pile1, GameStateIn, WinPile1),
  length(WinPile1, 0),
  put_assoc(game_phase, GameStateIn, playing, GameStateOut).


next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = restocking_player2,
  get_assoc(win_pile2, GameStateIn, WinPile2),
  length(WinPile2, 0),
  put_assoc(game_phase, GameStateIn, playing, GameStateOut).


next(GameStateIn, GameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = evaluating,
  get_assoc(deck1, GameStateIn, Deck1),
  get_assoc(deck2, GameStateIn, Deck2),
  \+ length(Deck1, 0),
  \+ length(Deck2, 0),
  put_assoc(game_phase, GameStateIn, playing, GameStateOut).



a_card_but_not(NotCard, PossibleCard) :-
  a_card(Card),
  Card \= NotCard,
  PossibleCard = possible(Card).


any_card(Card) :-
  Card = card(_,_,_,_,_,_,_).


sees(player1, GameStateIn, VisibleState) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = playing,
  get_assoc(deck1, GameStateIn, Deck1),
  get_assoc(deck2, GameStateIn, Deck2),
  get_assoc(win_pile1, GameStateIn, WinPile1),
  get_assoc(win_pile2, GameStateIn, WinPile2),
  get_assoc(cards_on_table, GameStateIn, CardsOnTable),
  get_assoc(selected_category, GameStateIn, SelectedCategory),
  get_assoc(player_turn, GameStateIn, PlayerTurn),
  nth0(0, Deck1, VisibleCard),
  nth0(0, VisibleDeck1, VisibleCard),
  same_length(Deck1, VisibleDeck1),
  same_length(Deck2, VisibleDeck2),
  same_length(WinPile1, VisibleWinPile1),
  same_length(WinPile2, VisibleWinPile2),
  maplist(any_card, VisibleDeck1),
  maplist(any_card, VisibleDeck2),
  maplist(any_card, VisibleWinPile1),
  maplist(any_card, VisibleWinPile2),
  list_to_assoc([
    deck1-VisibleDeck1,
    deck2-VisibleDeck2,
    win_pile1-VisibleWinPile1,
    win_pile2-VisibleWinPile2,
    cards_on_table-CardsOnTable,
    selected_category-SelectedCategory,
    game_phase-playing,
    player_turn-PlayerTurn
  ], VisibleState).


sees(player2, GameStateIn, VisibleState) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = playing,
  get_assoc(deck1, GameStateIn, Deck1),
  get_assoc(deck2, GameStateIn, Deck2),
  get_assoc(win_pile1, GameStateIn, WinPile1),
  get_assoc(win_pile2, GameStateIn, WinPile2),
  get_assoc(cards_on_table, GameStateIn, CardsOnTable),
  get_assoc(selected_category, GameStateIn, SelectedCategory),
  get_assoc(player_turn, GameStateIn, PlayerTurn),
  nth0(0, Deck2, VisibleCard),
  nth0(0, VisibleDeck2, VisibleCard),
  same_length(Deck1, VisibleDeck1),
  same_length(Deck2, VisibleDeck2),
  same_length(WinPile1, VisibleWinPile1),
  same_length(WinPile2, VisibleWinPile2),
  maplist(any_card, VisibleDeck1),
  maplist(any_card, VisibleDeck2),
  maplist(any_card, VisibleWinPile1),
  maplist(any_card, VisibleWinPile2),
  list_to_assoc([
    deck1-VisibleDeck1,
    deck2-VisibleDeck2,
    win_pile1-VisibleWinPile1,
    win_pile2-VisibleWinPile2,
    cards_on_table-CardsOnTable,
    selected_category-SelectedCategory,
    game_phase-playing,
    player_turn-PlayerTurn
  ], VisibleState).


sees(_, GameStateIn, VisibleState) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  get_assoc(deck1, GameStateIn, Deck1),
  get_assoc(deck2, GameStateIn, Deck2),
  get_assoc(win_pile1, GameStateIn, WinPile1),
  get_assoc(win_pile2, GameStateIn, WinPile2),
  get_assoc(cards_on_table, GameStateIn, CardsOnTable),
  get_assoc(selected_category, GameStateIn, SelectedCategory),
  get_assoc(player_turn, GameStateIn, PlayerTurn),
  same_length(Deck1, VisibleDeck1),
  same_length(Deck2, VisibleDeck2),
  same_length(WinPile1, VisibleWinPile1),
  same_length(WinPile2, VisibleWinPile2),
  maplist(any_card, VisibleDeck1),
  maplist(any_card, VisibleDeck2),
  maplist(any_card, VisibleWinPile1),
  maplist(any_card, VisibleWinPile2),
  list_to_assoc([
    deck1-VisibleDeck1,
    deck2-VisibleDeck2,
    win_pile1-VisibleWinPile1,
    win_pile2-VisibleWinPile2,
    cards_on_table-CardsOnTable,
    selected_category-SelectedCategory,
    game_phase-GamePhase,
    player_turn-PlayerTurn
  ], VisibleState).



player_options(player1, GameStateIn, PartialGameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = playing,
  get_assoc(player_turn, GameStateIn, PlayerTurn),
  PlayerTurn = player1,
  member(SelectedCategory, [
    category(0, distance),
    category(1, temp),
    category(2, orbit_time),
    category(3, radius),
    category(4, mass),
    category(5, earth_similarity)
  ]),
  list_to_assoc([selected_category-SelectedCategory], PartialGameStateOut).


player_options(player1, GameStateIn, PartialGameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = scoring,
  empty_assoc(PartialGameStateOut).


player_options(player2, GameStateIn, PartialGameStateOut) :-
  get_assoc(game_phase, GameStateIn, GamePhase),
  GamePhase = playing,
  get_assoc(player_turn, GameStateIn, PlayerTurn),
  PlayerTurn = player2,
  member(SelectedCategory, [
    category(0, distance),
    category(1, temp),
    category(2, orbit_time),
    category(3, radius),
    category(4, mass),
    category(5, earth_similarity)
  ]),
  list_to_assoc([selected_category-SelectedCategory], PartialGameStateOut).
