**Tournament Management System**
================================

A Rust implementation of a tournament management system, supporting single elimination, double elimination, and Swiss-system tournaments.

**Features**
------------

* **Single Elimination Tournaments**: Supports creation and simulation of single elimination tournaments, where players are eliminated after a single loss.
* **Double Elimination Tournaments**: Supports creation and simulation of double elimination tournaments, where players are eliminated after two losses.
* **Swiss-System Tournaments**: Supports creation and simulation of Swiss-system tournaments, where players are matched against each other based on their win-loss records.
* **Player Management**: Allows for creation and management of players, including tracking of player IDs, names, and scores.
* **Match Management**: Allows for creation and management of matches, including tracking of match winners and losers.
* **Tournament Simulation**: Simulates tournaments, including automatic pairing of players and determination of winners.
* **Match History**: Keeps track of match history, including winners and losers of each match.
* **Player Scores**: Keeps track of player scores, including wins and losses.

**Usage**
-----

To use this project, simply clone the repository and run the tests using `cargo test`. You can also include the project as a dependency in your own Rust project by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tournament-management = { git = "https://github.com/DuckyBlender/tournament-rs" }
```

**Example Usage**
--------------

Here is an example of how to create a single elimination tournament and simulate it:
```rust
let players = vec![
    Player { id: 1, name: "Player 1".to_string() },
    Player { id: 2, name: "Player 2".to_string() },
    Player { id: 3, name: "Player 3".to_string() },
    Player { id: 4, name: "Player 4".to_string() },
];

let mut tournament = Tournament::new(TournamentType::SingleElimination, players);

let winner = tournament.start();

println!("Tournament Winner: {}", winner.unwrap());
```
**Tests**
-----

The project includes a suite of tests to ensure the correctness of the implementation. You can run the tests using `cargo test`.

**Test Coverage**
--------------

The tests cover the following scenarios:

* Creation and simulation of single elimination tournaments
* Creation and simulation of double elimination tournaments
* Creation and simulation of Swiss-system tournaments
* Player management, including creation and deletion of players
* Match management, including creation and deletion of matches
* Tournament simulation, including automatic pairing of players and determination of winners
* Match history and player scores tracking

**License**
-------

This project is licensed under the MIT License. See the LICENSE file for details.

**Contributing**
------------

Contributions are welcome! If you'd like to contribute to this project, please fork the repository and submit a pull request.