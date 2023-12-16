use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

advent_of_code::solution!(7);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
    Joker,
}

impl Card {
    const ORDERING: [Self; 4] = [Card::Jack, Card::Queen, Card::King, Card::Ace];

    fn try_to_number(&self) -> Option<u8> {
        match self {
            Card::Number(n) => Some(*n),
            _ => None,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        } else if *self == Card::Joker {
            return Some(std::cmp::Ordering::Less);
        } else if *other == Card::Joker {
            return Some(std::cmp::Ordering::Greater);
        } else if let Some(order) = match (self.try_to_number(), other.try_to_number()) {
            (Some(self_number), Some(other_number)) => self_number.partial_cmp(&other_number),
            (Some(_), None) => Some(std::cmp::Ordering::Less),
            (None, Some(_)) => Some(std::cmp::Ordering::Greater),
            (None, None) => None,
        } {
            Some(order)
        } else {
            let self_index = Card::ORDERING
                .iter()
                .position(|x| x == self)
                .expect("Card not found in ordering");
            let other_index = Card::ORDERING
                .iter()
                .position(|x| x == other)
                .expect("Card not found in ordering");

            self_index.partial_cmp(&other_index)
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Ace => write!(f, "A"),
            Card::King => write!(f, "K"),
            Card::Queen => write!(f, "Q"),
            Card::Jack => write!(f, "J"),
            Card::Number(10) => write!(f, "T"),
            Card::Number(n) => write!(f, "{}", n),
            Card::Joker => write!(f, "J"),
        }
    }
}

impl Card {
    fn new(s: char, joker: bool) -> Result<Self, String> {
        match s {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(if joker { Card::Joker } else { Card::Jack }),
            'T' => Ok(Card::Number(10)),
            d => {
                let d: u32 = d.to_digit(10).ok_or("Invalid card value")?;

                if d >= 2 && d <= 9 {
                    Ok(Card::Number(d as u8))
                } else {
                    Err("Invalid card number".to_string())
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GameType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl GameType {
    const ORDERING: [Self; 7] = [
        GameType::HighCard,
        GameType::OnePair,
        GameType::TwoPair,
        GameType::ThreeOfAKind,
        GameType::FullHouse,
        GameType::FourOfAKind,
        GameType::FiveOfAKind,
    ];
}

impl PartialOrd for GameType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_index = GameType::ORDERING
            .iter()
            .position(|x| x == self)
            .expect("GameType not found in ordering");
        let other_index = GameType::ORDERING
            .iter()
            .position(|x| x == other)
            .expect("GameType not found in ordering");

        self_index.partial_cmp(&other_index)
    }
}

#[derive(Debug)]
struct Hand(Vec<Card>);

impl Hand {
    fn new(s: &str, joker: bool) -> Result<Hand, String> {
        let mut cards: Vec<Card> = Vec::new();

        for card in s.chars() {
            cards.push(Card::new(card, joker)?);
        }

        Ok(Hand(cards))
    }

    fn game_type(&self) -> GameType {
        let mut count: HashMap<Card, usize> = HashMap::new();

        for card in self.0.iter().cloned() {
            *count.entry(card).or_insert(0) += 1;
        }

        let mut top_cards = count
            .iter()
            .sorted_by_key(|(_, &count)| -1 * count as isize);

        let modality = {
            let (modal_card, modality) = top_cards.next().unwrap();

            // If top card is the Joker, then check if we have another top card, otherwise the
            // modality is forced to 0 (as the number of jokers is added to this later).
            let modality = if modal_card == &Card::Joker {
                if let Some((_, &modality)) = top_cards.next() {
                    modality
                } else {
                    0
                }
            } else {
                *modality
            };

            modality
        };

        let jokers = *count.get(&Card::Joker).unwrap_or(&0);
        let distinct_non_jokers = count.len() - if jokers > 0 { 1 } else { 0 };

        let top_card = modality + jokers;

        match top_card {
            5 => GameType::FiveOfAKind,
            4 => GameType::FourOfAKind,
            3 => {
                if distinct_non_jokers == 2 {
                    GameType::FullHouse
                } else {
                    GameType::ThreeOfAKind
                }
            }
            2 => {
                if distinct_non_jokers == 3 {
                    GameType::TwoPair
                } else {
                    GameType::OnePair
                }
            }
            _ => GameType::HighCard,
        }
    }

    // card_values maps cards to their worth. Jokers are worth 1.
    fn card_values(&self) -> Vec<u8> {
        self.0
            .iter()
            .map(|&card| {
                if card == Card::Joker {
                    1
                } else if let Card::Number(d) = card {
                    d
                } else {
                    Card::ORDERING.iter().position(|&x| x == card).unwrap() as u8 + 11
                }
            })
            .collect()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.0.len() == other.0.len());

        for (self_card, other_card) in self.0.iter().zip(other.0.iter()) {
            if self_card != other_card {
                return false;
            }
        }

        true
    }
}

fn sort_hands(a: &Hand, b: &Hand) -> Option<std::cmp::Ordering> {
    let (a_type, b_type) = (a.game_type(), b.game_type());

    if a_type != b_type {
        a_type.partial_cmp(&b_type)
    } else {
        a.partial_cmp(&b)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        assert!(self.0.len() == other.0.len());

        for (self_card, other_card) in self.card_values().iter().zip(other.card_values().iter()) {
            if let Some(order) = self_card.partial_cmp(other_card) {
                if order != std::cmp::Ordering::Equal {
                    return Some(order);
                }
            }
        }

        None
    }
}

fn parse(input: &str, joker: bool) -> Result<Vec<(Hand, u32)>, String> {
    input
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(" ").unwrap();
            Ok((Hand::new(cards, joker)?, bid.parse::<u32>().unwrap()))
        })
        .collect::<Result<Vec<_>, String>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<(Hand, u32)> = parse(input, false).unwrap();

    Some(
        games
            .iter()
            .sorted_by(|(a, _), (b, _)| sort_hands(a, b).unwrap())
            .enumerate()
            .map(|(i, (_, bid))| bid * (i as u32 + 1))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<(Hand, u32)> = parse(input, true).unwrap();

    Some(
        games
            .iter()
            .sorted_by(|(a, _), (b, _)| sort_hands(a, b).unwrap())
            .enumerate()
            .inspect(|(_, (_hand, _))| {
                #[cfg(debug_assertions)]
                if _hand.0.contains(&Card::Joker) {
                    println!("{:?}", _hand);
                }
            })
            .map(|(i, (_, bid))| bid * (i as u32 + 1))
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
