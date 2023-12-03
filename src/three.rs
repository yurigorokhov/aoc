use std::{fs::File, io::{self, BufRead}};

use clap::Args;

#[derive(Args, Debug)]
pub struct CommandThreeArgs {
   file: String,
   
   #[clap(long, short = '2', action)]
   two: bool
}

#[derive(PartialEq, Debug)]
struct TokenPosition {
    line_number: i32,
    start: i32,
    end: i32,
}

impl TokenPosition {
    fn intersect(&self, other: &Self) -> bool {
        if (self.line_number - other.line_number).abs() > 1 {
            return false
        }
        return other.start <= self.end + 1 && other.end >= self.start - 1
    }
}

#[derive(PartialEq, Debug)]
struct Token<T> {
    value: T,
    position: TokenPosition,
}

fn parse_tokens<I>(lines: I) -> (Vec<Token<u32>>, Vec<Token<char>>)
where
    I: Iterator<Item = String>,
{
    let mut numbers: Vec<Token<u32>> = Vec::new();
    let mut symbols: Vec<Token<char>> = Vec::new();
    let mut buffer = String::new();
    let mut buffer_start_idx: Option<u32> = None;

    let mut flush_number_buffer = |line_number: usize, char_idx: usize, buffer_start_idx: &mut Option<u32>, buffer: &mut String| {
        if let Some(start_idx) = *buffer_start_idx {
            numbers.push(
                Token::<u32> { 
                    value: buffer.parse::<u32>().unwrap(), 
                    position: TokenPosition { 
                        line_number: line_number as i32, 
                        start: start_idx as i32, 
                        end: (char_idx - 1) as i32
                    } 
                }
            );
            *buffer_start_idx = None;
            buffer.clear();
        }
    };

    for (line_number, line) in lines.enumerate() {
        let mut last_char_idx = 0;
        for (char_idx, ch) in line.chars().enumerate() {
            last_char_idx = char_idx;

            // if character is a number
            if ch.is_numeric() {
                buffer.push(ch);
                if buffer_start_idx == None {
                    buffer_start_idx = Some(char_idx as u32);
                }
            } else {
                // close out number parsing if buffer has a value
                flush_number_buffer(line_number, char_idx, &mut buffer_start_idx, &mut buffer);

                // if character is a symbol
                if !ch.is_numeric() && ch != '.' {
                    symbols.push(
                        Token::<char> {
                            value: ch,
                            position: TokenPosition { 
                                line_number: line_number as i32, 
                                start: char_idx as i32, 
                                end: char_idx as i32 
                            }
                        }
                    );
                }
            }
        }
        flush_number_buffer(line_number, last_char_idx, &mut buffer_start_idx, &mut buffer);
    }
    (numbers, symbols)
}

pub fn run(args: &CommandThreeArgs) -> u32 {
    let file = File::open(args.file.as_str())
        .expect("Should have been able to read the file");
    let lines = io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l.unwrap().to_string());
    let (numbers, symbols) = parse_tokens(lines);
    
    if !args.two {
        let sum = numbers
            .iter()
            .filter(|n| symbols.iter().any(|s| n.position.intersect(&s.position)))
            .map(|n| n.value)
            .sum();
        println!("The sum is: {}", sum);
        sum
    } else {
        let sum = symbols
            .iter()
            .filter(|s| s.value == '*')
            .map(|star| {
                let parts: Vec<&Token<u32>> = numbers
                    .iter()
                    .filter(|n| star.position.intersect(&n.position))
                    .collect::<Vec<&Token<u32>>>()
                    .into();

                // mulitply part values together
                if parts.len() < 2 {
                    return 0
                }
                parts.iter().map(|p| p.value).reduce(|a, e| a * e).unwrap() as u32
            })
            .sum();
        println!("The sum is: {}", sum);
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            self::run(&CommandThreeArgs{file: "./inputs/three_test.txt".to_string(), two: false}),
            4361,
        );
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(
            self::run(&CommandThreeArgs{file: "./inputs/three_test.txt".to_string(), two: true}),
            467835,
        );
    }

    #[test]
    fn test_parse_tokens() {
        let (numbers, symbols) = parse_tokens([String::from("1234...*..!")].into_iter());
        assert_eq!(numbers.len(), 1);
        assert_eq!(
            numbers.first(), 
            Some(&Token::<u32>{ 
                value: 1234, 
                position: TokenPosition { 
                    line_number: 0, 
                    start: 0,
                    end: 3
                }
            })
        );
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols.first(), Some(
            &Token::<char> {
                value: '*',
                position: TokenPosition{ line_number: 0, start: 7, end: 7 }
            }
        ));
        assert_eq!(symbols.last(), Some(
            &Token::<char> {
                value: '!',
                position: TokenPosition{ line_number: 0, start: 10, end: 10 }
            }
        ));
    }
}