use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

use clap::Args;

#[derive(Args, Debug)]
pub struct CommandOneArgs {
    file: String,

    #[clap(long, short = '2', action)]
    two: bool,
}

const RADIX: u32 = 10;
const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_line(line: String, part_two: bool) -> Result<u32, Box<dyn Error>> {
    let mut i = 0;
    let mut line_numbers: Vec<u32> = vec![];
    while i < line.len() {
        if part_two {
            for (j, &digit) in DIGITS.iter().enumerate() {
                let len = digit.len();
                if len <= (line.len() - i) && line[i..i + len] == *digit {
                    line_numbers.push(j as u32 + 1);
                    break;
                }
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
        return Err(format!("Could not parse line: {}", line).into());
    }
    Ok(line_numbers.first().unwrap() * RADIX + line_numbers.last().unwrap())
}

pub fn run(args: &CommandOneArgs) -> u32 {
    let file = File::open(args.file.as_str()).expect("Should have been able to read the file");
    let lines = io::BufReader::new(file).lines();
    let sum: u32 = lines
        .into_iter()
        .filter_map(|line| parse_line(line.unwrap(), args.two).ok())
        .sum();
    println!("The sum is: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(
            run(&CommandOneArgs {
                file: String::from("./inputs/one_test.txt"),
                two: false
            }),
            209
        );

        assert_eq!(
            run(&CommandOneArgs {
                file: String::from("./inputs/one_test.txt"),
                two: true
            }),
            281
        );
    }
}
