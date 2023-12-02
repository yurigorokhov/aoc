use std::{fs::File, io::{self, BufRead}, cmp::Ordering};

use clap::Args;

#[derive(Args, Debug)]
pub struct CommandTwoArgs {
   file: String,
   
   #[clap(long, short = '2', action)]
   two: bool
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    configurations: Vec<GameConfiguration>
}

impl Game {
    pub fn from_str(value: String) -> Self {
        let split: Vec<&str> = value.split(":").collect();
        let game_id = split[0][5..].parse::<u32>().unwrap();
        Game{ 
            id: game_id, 
            configurations: split[1].split(";").map(|g| GameConfiguration::from_str(g.trim().to_string())).collect()
        }
    }

    fn min(&self) -> GameConfiguration {
        GameConfiguration { 
            red: self.configurations.iter().map(|c| c.red).max().unwrap(),
            green: self.configurations.iter().map(|c| c.green).max().unwrap(),
            blue: self.configurations.iter().map(|c| c.blue).max().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct GameConfiguration {
    red: u32,
    green: u32,
    blue: u32,
}

impl PartialOrd for GameConfiguration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self.red >= other.red && self.green >= other.green && self.blue >= other.blue {
            return Some(Ordering::Greater)
        }
        if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            return Some(Ordering::Less)
        }
        None
    }
}

impl GameConfiguration {
    pub fn from_str(value: String) -> Self {
        let mut r = 0u32;
        let mut g = 0u32;
        let mut b = 0u32;
        value
            .trim()
            .split(",")
            .map(|s| s.trim().to_lowercase())
            .for_each(|s| {
                let vals: Vec<&str> = s.split(" ").collect();
                match vals[..] {
                    [count, "red"] => r = count.parse::<u32>().unwrap(),
                    [count, "green"] => g = count.parse::<u32>().unwrap(),
                    [count, "blue"] => b = count.parse::<u32>().unwrap(),
                    _ => panic!("Could not parse")
                }
            });
        Self { red: r, green: g, blue: b }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}


pub fn run(args: &CommandTwoArgs) -> u32 {
    let file = File::open(args.file.as_str())
        .expect("Should have been able to read the file");
    let lines = io::BufReader::new(file).lines();
    
    if !args.two {
        // 12 red cubes, 13 green cubes, and 14
        let test_configuration = GameConfiguration { red: 12, green: 13, blue: 14 };
        let sum: u32 = lines
            .into_iter()
            .map(|line| Game::from_str(line.unwrap()))
            .filter(|g| g.configurations.iter().all(|c| c <= &test_configuration))
            .map(|g| g.id)
            .sum();
        println!("The sum is: {}", sum);
        sum
    } else {
        let sum: u32 = lines
            .into_iter()
            .map(|line| Game::from_str(line.unwrap()).min().power())
            .sum();
        println!("The sum is: {}", sum);
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_configuration_from_str() {
        assert_eq!(
            GameConfiguration::from_str("1 blue, 2 green".to_string()),
            GameConfiguration{ red: 0, blue: 1, green: 2 },
        );
    }

    #[test]
    fn game_configuration_ordering() {
        assert!(
            GameConfiguration{ red: 0, blue: 1, green: 2 } < GameConfiguration{ red: 1, blue: 1, green: 2 }
        );
        assert!(
            GameConfiguration{ red: 1, blue: 1, green: 2 } == GameConfiguration{ red: 1, blue: 1, green: 2 }
        );
        assert!(
            GameConfiguration{ red: 1, blue: 2, green: 2 } > GameConfiguration{ red: 1, blue: 1, green: 2 }
        );
    }

    #[test]
    fn game_from_str() {
        assert_eq!(
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
            Game { id: 1, configurations: vec![
                GameConfiguration { red: 4, green: 0, blue: 3 },
                GameConfiguration { red: 1, green: 2, blue: 6 },
                GameConfiguration { red: 0, green: 2, blue: 0 },
            ] },
        );
    }
}