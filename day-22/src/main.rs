#![feature(str_split_once)]

use std::{collections::{HashSet, VecDeque}, error::Error, fs::read_to_string};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type Card = u64;
type Deck = VecDeque<Card>;

#[derive(Debug)]
struct Combat {
    id: u64,
    first_deck: Deck,
    second_deck: Deck,
    history: HashSet<(Deck, Deck)>,
    winner: Option<bool>,
}

impl Combat {
    fn from_str(input: &str) -> Result<Self> {
        let (first, second) = input.split_once("\n\n").ok_or("invalid input")?;
        let first_deck = first.lines().skip(1).map(|card| card.parse::<Card>().unwrap()).collect();
        let second_deck = second.lines().skip(1).map(|card| card.parse::<Card>().unwrap()).collect();

        Ok(Combat{ id: 0, first_deck, second_deck, winner: None, history: HashSet::new() })
    }

    fn to_subgame(&self, p1_card: Card, p2_card: Card) -> Self {
        Combat {
            id: self.id + 1,
            first_deck: self.first_deck.iter().take(p1_card as usize).cloned().collect(),
            second_deck: self.second_deck.iter().take(p2_card as usize).cloned().collect(),
            history: self.history.clone(),
            winner: None,
        }
    }

    fn regular_round(&mut self, p1_card: Option<Card>, p2_card: Option<Card>) -> bool {
        match (p1_card, p2_card) {
            (Some(a), Some(b)) if a != b => {
                // regular round
                if a > b {
                    self.first_deck.push_back(a);
                    self.first_deck.push_back(b);
                } else {
                    self.second_deck.push_back(b);
                    self.second_deck.push_back(a);
                }

                true
            },
            (Some(_), None) | (None, Some(_)) => {
                // no cards left for one player
                let is_p1_winning = p1_card.is_some() && p2_card.is_none();
                self.winner = Some(is_p1_winning);
                
                if is_p1_winning {
                    self.first_deck.push_front(p1_card.unwrap());
                } else {
                    self.second_deck.push_front(p2_card.unwrap());
                }

                false
            },
            _ => panic!("invalid state"),
        }
    }

    fn advance_round(&mut self) -> bool {
        let p1_card = self.first_deck.pop_front();
        let p2_card = self.second_deck.pop_front();
        self.regular_round(p1_card, p2_card)
    }

    fn advance_recursive_round(&mut self) -> bool {
        let current_state: (Deck, Deck) = (self.first_deck.clone(), self.second_deck.clone());

        if self.history.contains(&current_state) {
            // winner by recursion
            self.winner = Some(true); // P1 wins
            return false;
        }

        self.history.insert(current_state);

        match (self.first_deck.pop_front(), self.second_deck.pop_front()) {
            (Some(a), Some(b)) if a as usize <= self.first_deck.len() && b as usize <= self.second_deck.len() => {
                // subgame
                let mut subcombat = self.to_subgame(a, b);

                let is_p1_winning_subcombat = loop {
                    if !subcombat.advance_recursive_round() {
                        break subcombat.winner.expect("there must be a winner of the sub-combat");
                    }
                };

                if is_p1_winning_subcombat {
                    self.first_deck.push_back(a);
                    self.first_deck.push_back(b);
                } else {
                    self.second_deck.push_back(b);
                    self.second_deck.push_back(a);
                }

                true
            },
            (p1_card, p2_card) => self.regular_round(p1_card, p2_card),
        }
    }

    fn score(&self) -> u64 {
        if self.winner.unwrap() {
            self.first_deck.iter()
        } else {
            self.second_deck.iter()
        }.rev().enumerate().map(|(i, card)| (i as u64 + 1) * *card).sum()
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;
    let mut combat = Combat::from_str(&input)?;
    
    let score = loop {
        if !combat.advance_round() {
            break combat.score();
        }
    };

    println!("Part 1: {}", score);
    
    let mut combat = Combat::from_str(&input)?;
    
    let score = loop {
        if !combat.advance_recursive_round() {
            break combat.score();
        }
    };

    println!("Part 2: {}", score);

    Ok(())
}
