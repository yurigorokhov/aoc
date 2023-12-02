use std::{error::Error, fs};

use clap::Args;

#[derive(Args, Debug)]
pub struct CommandOneArgs {
   file: String,
}

const RADIX: u32 = 10;
const DIGITS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn parse_line(line: &str) -> Result<u32, Box<dyn Error>> {
    let mut i = 0;
    let mut line_numbers: Vec<u32> = vec![];
    while i < line.len() {
        for (j, &digit) in DIGITS.iter().enumerate() {
            let len = digit.len();
            if len <= (line.len() - i) && line[i..i + len] == *digit {
                line_numbers.push(j as u32 + 1);
                break;
            }
        }
        let c = line.chars().nth(i).unwrap();
        match c.to_digit(RADIX) {
            Some(d) => line_numbers.push(d),
            None => {}
        }
        i += 1
    }
    if line_numbers.len() == 0 {
        return Err(format!("Could not parse line: {}", line).into())
    }
    Ok(line_numbers.first().unwrap() * RADIX + line_numbers.last().unwrap())
}

pub fn run(args: &CommandOneArgs) {
    let contents = fs::read_to_string(args.file.as_str())
        .expect("Should have been able to read the file");
    let sum: u32 = contents
        .split("\n")
        .filter_map(|line| parse_line(line).ok())
        .sum();
    println!("The sum is: {}", sum);
}