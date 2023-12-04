use std::{fs::File, io::{self, BufRead}};
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

impl Card {
    pub fn new_from_line(line: String) -> Self {
        let split: Vec<_> = line.split(&[':', '|'][..]).collect();
        assert!(split.len() == 3);
        Card {
            number: split[0][4..].trim().parse::<u32>().unwrap(),
            winning: split[1].split_whitespace().into_iter().map(|i| i.parse::<i32>().unwrap()).collect(),
            draw: split[2].split_whitespace().into_iter().map(|i| i.parse::<i32>().unwrap()).collect(),
        }
    }

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

pub fn run(args: &CommandFourArgs) -> u32 {
    let file = File::open(args.file.as_str())
        .expect("Should have been able to read the file");
    let cards: Vec<Card> = io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| Card::new_from_line(l.unwrap()))
        .collect();
    
    if !args.two {
        let sum = cards.iter().map(|c| c.score()).sum();
        println!("The sum is: {}", sum);
        sum
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
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            self::run(&CommandFourArgs{file: "./inputs/four_test.txt".to_string(), two: false}),
            13,
        );
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(
            self::run(&CommandFourArgs{file: "./inputs/four_test.txt".to_string(), two: true}),
            30,
        );
    }

    #[test]
    fn test_new_from_line() {
        assert_eq!(
            Card::new_from_line(String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")),
            Card {
                number: 1,
                winning: vec![41, 48, 83, 86, 17],
                draw: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
        );
    }
}