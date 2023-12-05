use thiserror::Error;
use anyhow::Error as AnyError;
use anyhow::Result as AnyResult;

use std::{fs::File, io::{self, BufRead}, str::FromStr};
use std::collections::VecDeque;

use clap::Args;

#[derive(Args, Debug)]
pub struct CommandFourArgs {
   file: String,
   
   #[clap(long, short = '2', action)]
   two: bool
}

#[derive(Debug, PartialEq)]
struct Card {
    number: u32,
    winning: Vec<i32>,
    draw: Vec<i32>,
}

#[derive(Error, Debug, PartialEq)]
pub enum CardParseError {
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Failed to parse card from string: `{0}`")]
    FormatError(String)
}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split(&[':', '|'][..]).collect();
        if split.len() != 3 {
            return Err(CardParseError::FormatError(s.to_string()));
        }
        Ok(Card {
            number: split[0][4..].trim().parse::<u32>()?,
            winning: split[1]
                .split_whitespace()
                .into_iter()
                .map(|i| i.parse::<i32>())
                .collect::<Result<Vec<i32>, std::num::ParseIntError>>()?,
            draw: split[2]
                .split_whitespace()
                .into_iter()
                .map(|i| i.parse::<i32>())
                .collect::<Result<Vec<i32>, std::num::ParseIntError>>()?,
        })
    }
}

impl Card {
    fn number_matches(&self) -> u32 {
        self.draw.iter().filter(|n| self.winning.contains(n)).count() as u32
    }
 
    fn score(&self) -> u32 {
        let n_wins = self.number_matches();
        if n_wins == 0 {
            0
        } else {
            2u32.pow((n_wins - 1) as u32)
        }
    }
}

pub fn run(args: &CommandFourArgs) -> AnyResult<u32> {
    let file = File::open(args.file.as_str())
        .expect("Should have been able to read the file");
    let cards: Vec<Card> = io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l?.as_str().parse::<Card>().map_err(|e| AnyError::from(e)))
        .collect::<Result<Vec<Card>, AnyError>>()?;
    
    if !args.two {
        let sum = cards.iter().map(|c| c.score()).sum();
        println!("The sum is: {}", sum);
        Ok(sum)
    } else {
        let mut queue: VecDeque<&Card> = VecDeque::new();
        for card in cards.iter() {
            queue.push_back(card);
        }

        let mut count = 0;
        while let Some(c) = queue.pop_front() {
            count += 1;
            for i in 1..=c.number_matches() {
                queue.push_back(&cards[(c.number + i - 1) as usize])
            }
        }
        println!("The sum is: {}", count);
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let r = self::run(&CommandFourArgs{file: "./inputs/four_test.txt".to_string(), two: false});
        assert!(r.is_ok());
        assert_eq!(
            r.unwrap(),
            13,
        );
    }

    #[test]
    fn test_input_part_two() {
        let r = self::run(&CommandFourArgs{file: "./inputs/four_test.txt".to_string(), two: true});
        assert_eq!(
            r.unwrap(),
            30,
        );
    }

    #[test]
    fn test_new_from_line() {
        assert_eq!(
            Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Ok(Card {
                number: 1,
                winning: vec![41, 48, 83, 86, 17],
                draw: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }),
        );
    }

    #[test]
    fn test_new_from_line_error() {
        let r = Card::from_str("Card a: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert!(r.is_err());
        assert_eq!(r.err().unwrap().to_string(), "invalid digit found in string");

        let r = Card::from_str("Card 1: a 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert!(r.is_err());
        assert_eq!(r.err().unwrap().to_string(), "invalid digit found in string");
    }
}