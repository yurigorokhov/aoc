use anyhow::anyhow;
use itertools::Itertools;

use anyhow::Error as AnyError;
use anyhow::Result as AnyResult;

use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::BufRead;

use clap::Args;

#[derive(Args, Debug)]
pub struct CommandSevenArgs {
    file: String,

    #[clap(long, short = '2', action)]
    two: bool,
}

const CARDS: &str = "AKQJT98765432";

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card {
    rank: u32,
    val: char,
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.val)
    }
}

impl Card {
    fn new(val: char, j_as_joker: bool) -> Option<Self> {
        let index = CARDS.find(val)?;
        Some(Card{
            val: val,
            rank: match val {
                'J' if j_as_joker => 0,
                _ => 100 - index as u32
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPair = 4,
    OnePair = 5,
    HighCard = 6,
}

struct Hand {
    cards: Vec<Card>,
    points: u32,
    j_as_joker: bool,
}

impl Hand {
    fn from_str(s: &str, j_as_joker: bool) -> Result<Self, AnyError> {
        let split_line: Vec<&str> = s.split(" ").collect();
        let points = split_line[1].parse::<u32>()?;
        let cards = split_line[0].chars()
            .map(|c| Card::new(c, j_as_joker).ok_or(anyhow!("")))
            .collect::<Result<Vec<Card>, AnyError>>()?;
        Ok(Hand { cards: cards, points: points, j_as_joker: j_as_joker })
    }

    fn get_type(&self) -> HandType {
        let mut map: HashMap<char, u32> = self.cards
            .iter()
            .into_grouping_map_by(|&x| x.val)
            .fold(0, |acc, _key, _value| acc + 1);

        let mut joker_count: u32 = 0;
        if self.j_as_joker {
            if let Some(count) = map.get(&'J') {
                joker_count = *count;
                map.remove(&'J');
            }
        }

        let mut counts: Vec<u32> = map
            .values()
            .sorted()
            .rev()
            .map(|&x| x)
            .collect();

        // this can only happen if they are all jokers!
        if counts.len() == 0 {
            return HandType::FiveOfAKind
        }
        counts[0] += joker_count;

        match counts.iter().join("").as_str() {
            "5" => HandType::FiveOfAKind,
            "41" => HandType::FourOfAKind,
            "32" => HandType::FullHouse,
            "311" => HandType::ThreeOfAKind,
            "221" => HandType::TwoPair,
            "2111" => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut cmp = self.get_type().cmp(&other.get_type()).reverse();
        let mut card_idx = 0;
        while cmp == std::cmp::Ordering::Equal && card_idx < self.cards.len() {
            cmp = self.cards[card_idx].cmp(&other.cards[card_idx]);
            card_idx += 1;
        }
        cmp
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hand")
            .field("cards", &self.cards.iter().map(|c| c.val).join(""))
            .field("points", &self.points)
            .field("hand", &self.get_type())
            .finish()
    }
}

pub fn run(args: &CommandSevenArgs) -> AnyResult<u64> {
    let file = File::open(args.file.as_str()).expect("Should have been able to read the file");
    let lines = io::BufReader::new(file)
        .lines()
        .filter_map(|s| s.ok())
        .into_iter();
    
    if !args.two {
        let hands: Vec<Hand> = lines.map(|l| Hand::from_str(&l, false)).collect::<Result<Vec<Hand>, AnyError>>()?;
        let mut points: u64 = 0;
        for (i, h) in hands.iter().sorted().enumerate() {
            points += ((i+1) as u64) * h.points as u64;
        }
        println!("Answer: {}", points);
        Ok(points)
    } else {
        let hands: Vec<Hand> = lines.map(|l| Hand::from_str(&l, true)).collect::<Result<Vec<Hand>, AnyError>>()?;
        let mut points: u64 = 0;
        for (i, h) in hands.iter().sorted().enumerate() {
            points += ((i+1) as u64) * h.points as u64;
        }
        println!("Answer: {}", points);
        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let r = self::run(&CommandSevenArgs {
            file: "./inputs/seven_test.txt".to_string(),
            two: false,
        });
        assert_eq!(r.unwrap(), 6440);
    }

    #[test]
    fn test_input_part2() {
        let r = self::run(&CommandSevenArgs {
            file: "./inputs/seven_test.txt".to_string(),
            two: true,
        });
        assert_eq!(r.unwrap(), 5905);
    }
}
