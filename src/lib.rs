#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]

use std::fmt;
use std::collections::HashMap;
use rand::Rng;

/// Represents a player in the tournament.
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Player {
    pub id: u32,
    pub name: String,
}

/// Represents a match between two players.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Match {
    pub player1: Player,
    pub player2: Player,
    pub winner: Option<Player>,
}

/// Enum to represent the type of tournament.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TournamentType {
    SingleElimination,
    DoubleElimination,
    Swiss,
}

/// Represents a tournament.
#[derive(Debug, PartialEq, Eq)]
pub struct Tournament {
    pub tournament_type: TournamentType,
    pub players: Vec<Player>,
    pub matches: Vec<Match>,
}

/// Helper struct to hold round results.
struct RoundResult {
    winners: Vec<Player>,
    losers: Vec<Player>,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (ID: {})", self.name, self.id)
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.winner {
            Some(winner) => write!(
                f,
                "{} vs {} - Winner: {}",
                self.player1, self.player2, winner
            ),
            None => write!(f, "{} vs {} - No winner yet", self.player1, self.player2),
        }
    }
}

impl Tournament {
    /// Creates a new tournament.
    ///
    /// # Arguments
    ///
    /// * `tournament_type` - The type of the tournament.
    /// * `players` - A vector of players participating in the tournament.
    ///
    /// # Returns
    ///
    /// A new `Tournament` instance.
    #[must_use] pub fn new(tournament_type: TournamentType, players: Vec<Player>) -> Self {
        Self {
            tournament_type,
            players,
            matches: Vec::new(),
        }
    }

    /// Starts the tournament.
    ///
    /// # Returns
    ///
    /// The winner of the tournament, if any.
    pub fn start(&mut self) -> Option<Player> {
        match self.tournament_type {
            TournamentType::SingleElimination => self.start_single_elimination(),
            TournamentType::DoubleElimination => self.start_double_elimination(),
            TournamentType::Swiss => self.start_swiss(),
        }
    }

    /// Starts a single elimination tournament.
    ///
    /// # Returns
    ///
    /// The winner of the tournament, if any.
    fn start_single_elimination(&mut self) -> Option<Player> {
        let mut rng = rand::thread_rng(); // RNG for random number generation
        let mut round_players = self.players.clone();
        while round_players.len() > 1 {
            let mut next_round_players = Vec::new();
            for chunk in round_players.chunks(2) {
                if chunk.len() == 2 {
                    let winner_index = rng.gen_range(0..2); // Randomly select 0 or 1
                    let winner = chunk[winner_index].clone(); // Select winner based on random index
                    let match_ = Match {
                        player1: chunk[0].clone(),
                        player2: chunk[1].clone(),
                        winner: Some(winner.clone()),
                    };
                    self.matches.push(match_);
                    next_round_players.push(winner);
                } else {
                    next_round_players.push(chunk[0].clone());
                }
            }
            round_players = next_round_players;
        }
        round_players.first().cloned()
    }

    /// Starts a double elimination tournament.
    ///
    /// # Returns
    ///
    /// The winner of the tournament, if any.
    fn start_double_elimination(&mut self) -> Option<Player> {
        let mut winners_bracket = self.players.clone();
        let mut losers_bracket = Vec::new();
        let mut final_winner = None;
        while winners_bracket.len() > 1 || losers_bracket.len() > 1 {
            let winners_round_result = self.play_round(&winners_bracket);
            winners_bracket = winners_round_result.winners;
            losers_bracket.extend(winners_round_result.losers);
            if !losers_bracket.is_empty() {
                let losers_round_result = self.play_round(&losers_bracket);
                losers_bracket = losers_round_result.winners;
                // Losers of losers bracket are eliminated, not added back
            }
            // If one player left in each bracket, they face off
            if winners_bracket.len() == 1 && losers_bracket.len() == 1 {
                let final_match_winner = self.simulate_match(&winners_bracket[0], &losers_bracket[0]);
                if final_match_winner == losers_bracket[0] {
                    // Loser's bracket winner must win twice, play another match
                    final_winner = Some(self.simulate_match(&winners_bracket[0], &losers_bracket[0]));
                } else {
                    final_winner = Some(final_match_winner);
                }
                break;
            }
        }
        final_winner.or_else(|| winners_bracket.first().cloned())
    }

    /// Starts a Swiss-system tournament.
    ///
    /// # Returns
    ///
    /// The winner of the tournament, if any.
    fn start_swiss(&mut self) -> Option<Player> {
        let rounds = self.players.len().next_power_of_two().trailing_zeros();
        let mut scores = self
            .players
            .iter()
            .map(|p| (p.clone(), 0))
            .collect::<HashMap<Player, i32>>();
        for round in 0..rounds {
            println!("Round {}:", round + 1);
            let round_matches = Self::pair_players_swiss(&scores);
            for (player1, player2) in round_matches {
                let winner = self.simulate_match(&player1, &player2);
                *scores.entry(winner).or_insert(0) += 1;
            }
            Self::print_leaderboard(&scores);
        }
        scores
            .into_iter()
            .max_by_key(|&(_, score)| score)
            .map(|(player, _)| player)
    }

    /// Plays a round of matches.
    ///
    /// # Arguments
    ///
    /// * `players` - A slice of players participating in the round.
    ///
    /// # Returns
    ///
    /// A `RoundResult` containing the winners and losers of the round.
    fn play_round(&mut self, players: &[Player]) -> RoundResult {
        let mut rng = rand::thread_rng();
        let mut winners = Vec::new();
        let mut losers = Vec::new();
        for chunk in players.chunks(2) {
            if chunk.len() == 2 {
                let winner_index = rng.gen_range(0..2);
                let winner = chunk[winner_index].clone();
                let loser = chunk[1 - winner_index].clone();
                winners.push(winner.clone());
                losers.push(loser);
                // Optionally, record the match
                self.matches.push(Match {
                    player1: chunk[0].clone(),
                    player2: chunk[1].clone(),
                    winner: Some(winner.clone()),
                });
            } else {
                // Odd player out automatically advances
                winners.push(chunk[0].clone());
            }
        }
        RoundResult { winners, losers }
    }

    /// Simulates a match between two players.
    ///
    /// # Arguments
    ///
    /// * `player1` - The first player.
    /// * `player2` - The second player.
    ///
    /// # Returns
    ///
    /// The winner of the match.
    fn simulate_match(&mut self, player1: &Player, player2: &Player) -> Player {
        let mut rng = rand::thread_rng();
        let winner_index = rng.gen_range(0..2);
        let winner = if winner_index == 0 { player1 } else { player2 };
        // Record the match
        self.matches.push(Match {
            player1: player1.clone(),
            player2: player2.clone(),
            winner: Some(winner.clone()),
        });
        winner.clone()
    }

    /// Records a match with a specified winner.
    ///
    /// # Arguments
    ///
    /// * `player1` - The first player.
    /// * `player2` - The second player.
    /// * `winner` - The winner of the match.
    pub fn play_match(&mut self, player1: &Player, player2: &Player, winner: &Player) {
        // Record the match
        self.matches.push(Match {
            player1: player1.clone(),
            player2: player2.clone(),
            winner: Some(winner.clone()),
        });
    }

    /// Pairs players for a Swiss-system round based on their scores.
    ///
    /// # Arguments
    ///
    /// * `scores` - A hashmap of players and their scores.
    ///
    /// # Returns
    ///
    /// A vector of player pairs for the round.
    fn pair_players_swiss(scores: &HashMap<Player, i32>) -> Vec<(Player, Player)> {
        let mut players_sorted: Vec<_> = scores.iter().collect();
        players_sorted.sort_by_key(|&(_, &score)| -score);
        players_sorted
            .chunks(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    Some((chunk[0].0.clone(), chunk[1].0.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Prints the leaderboard of the tournament.
    ///
    /// # Arguments
    ///
    /// * `scores` - A hashmap of players and their scores.
    fn print_leaderboard(scores: &HashMap<Player, i32>) {
        let mut leaderboard: Vec<_> = scores.iter().collect();
        leaderboard.sort_by_key(|&(_, &score)| -score);
        println!("Leaderboard:");
        for (player, score) in leaderboard {
            println!("{player} - {score} points");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Helper function to create a list of players.
    ///
    /// # Arguments
    ///
    /// * `num` - The number of players to create.
    ///
    /// # Returns
    ///
    /// A vector of `Player` instances.
    fn create_players(num: u32) -> Vec<Player> {
        (1..=num)
            .map(|i| Player {
                id: i,
                name: format!("Player {i}"),
            })
            .collect()
    }

    #[test]
    fn test_single_elimination() {
        let players = create_players(8);
        let mut tournament = Tournament::new(TournamentType::SingleElimination, players.clone());
        let winner = tournament.start();
        assert!(winner.is_some());
        println!("Single Elimination Winner: {}", winner.unwrap());
        for match_ in &tournament.matches {
            println!("{match_}");
        }
        // Step-by-step verification
        let expected_matches = players.len() - 1; // 8 players -> 7 matches
        assert_eq!(tournament.matches.len(), expected_matches);
        for match_ in &tournament.matches {
            assert!(match_.winner.is_some());
        }
    }

    #[test]
    fn test_double_elimination() {
        let players = create_players(8);
        let mut tournament = Tournament::new(TournamentType::DoubleElimination, players.clone());
        let winner = tournament.start();
        assert!(winner.is_some());
        println!("Double Elimination Winner: {}", winner.unwrap());
        for match_ in &tournament.matches {
            println!("{match_}");
        }
        // Step-by-step verification
        // The number of matches can vary due to the double elimination structure
        assert!(tournament.matches.len() >= 7); // Minimum matches for 8 players
        for match_ in &tournament.matches {
            assert!(match_.winner.is_some());
        }
        // Show winner and loser brackets
        let mut winners_bracket_matches = Vec::new();
        let mut losers_bracket_matches = Vec::new();
        let mut winners_bracket_players = players;
        // let mut losers_bracket_players = Vec::new();
        for match_ in &tournament.matches {
            if winners_bracket_players.contains(&match_.player1) && winners_bracket_players.contains(&match_.player2) {
                winners_bracket_matches.push(match_);
                if let Some(winner) = &match_.winner {
                    winners_bracket_players.retain(|p| p != winner);
                    // losers_bracket_players.push(match_.player1.clone());
                    // losers_bracket_players.push(match_.player2.clone());
                    // losers_bracket_players.retain(|p| p != winner);
                }
            } else {
                losers_bracket_matches.push(match_);
            }
        }
        println!("Winners Bracket:");
        for match_ in &winners_bracket_matches {
            println!("{match_}");
        }
        println!("Losers Bracket:");
        for match_ in &losers_bracket_matches {
            println!("{match_}");
        }
    }

    #[test]
    fn test_swiss() {
        let players = create_players(8);
        let mut tournament = Tournament::new(TournamentType::Swiss, players.clone());
        let winner = tournament.start();
        assert!(winner.is_some());
        println!("Swiss Winner: {}", winner.unwrap());
        for match_ in &tournament.matches {
            println!("{match_}");
        }
        // Step-by-step verification
        // Swiss system should have log2(n) rounds, where n is the number of players
        let rounds = players.len().next_power_of_two().trailing_zeros();
        assert_eq!(tournament.matches.len(), rounds as usize * (players.len() / 2));
        for match_ in &tournament.matches {
            assert!(match_.winner.is_some());
        }
        // Show players' scores
        let mut scores = HashMap::new();
        for player in &players {
            scores.insert(player.clone(), 0);
        }
        for match_ in &tournament.matches {
            if let Some(winner) = &match_.winner {
                *scores.entry(winner.clone()).or_insert(0) += 1;
            }
        }
        println!("Players' scores: {scores:?}");
    }
}