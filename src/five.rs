use anyhow::bail;
use anyhow::Error as AnyError;
use anyhow::Result as AnyResult;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::vec;

use clap::Args;

#[derive(Args, Debug)]
pub struct CommandFiveArgs {
    file: String,

    #[clap(long, short = '2', action)]
    two: bool,
}

#[derive(Debug)]
struct MappingCollection {
    mappings: Vec<Mapping>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn intersection(&self, other: &Self) -> Option<Range> {
        if self.end < other.start || other.end < self.start {
            return None;
        }
        Some(Range {
            start: std::cmp::max(self.start, other.start),
            end: std::cmp::min(self.end, other.end),
        })
    }

    fn len(&self) -> u64 {
        self.end - self.start
    }

    fn subtract(&self, other: &Self) -> Vec<Range> {
        let mut results = vec![];
        if self == other {
            return results;
        }
        if let Some(inter) = self.intersection(other) {
            if inter.start > self.start {
                results.push(Range {
                    start: self.start,
                    end: inter.start,
                });
            }
            if inter.end < self.end {
                results.push(Range {
                    start: inter.end,
                    end: self.end,
                })
            }
        } else {
            results.push(self.clone());
        }
        // println!("SUBTRACT\n\n{:#?} - {:#?} = {:#?}\n\n", self, other, results);
        results
    }
}

impl MappingCollection {
    fn from_lines<T: Iterator<Item = String>>(iter: &mut T) -> AnyResult<Self> {
        let mut mappings: Vec<Mapping> = vec![];
        while let Some(line) = iter.next() {
            if line.trim().len() == 0 || line.contains("map") {
                break;
            }
            mappings.push(line.as_str().parse::<Mapping>().unwrap());
        }
        if mappings.len() == 0 {
            bail!("No mappings found");
        }
        Ok(MappingCollection { mappings: mappings })
    }

    fn map(&self, elem: u64) -> u64 {
        for mapping in self.mappings.iter() {
            if elem >= mapping.src.start && elem < mapping.src.end {
                return mapping.dst.start + (elem - mapping.src.start);
            }
        }
        return elem;
    }

    fn map_range(&self, range: Range) -> Vec<Range> {
        let mut remainders = vec![range];
        let mut mapped = vec![];
        for mapping in self.mappings.iter() {
            // println!("MAPPING \n\n {:#?} -> {:#?}", mapping.src, mapping.dst);
            let mut new_remainders = vec![];
            for r in remainders {
                let intersection = r.intersection(&mapping.src);
                let remainder = match intersection {
                    Some(intersection_range) => {
                        // map matching range to output range
                        let start =
                            mapping.dst.start + (intersection_range.start - mapping.src.start);
                        mapped.push(Range {
                            start: start,
                            end: start + intersection_range.len(),
                        });

                        // remainder
                        r.subtract(&intersection_range)
                    }
                    None => {
                        vec![r]
                    }
                };
                // println!("INTERSECT\n\n{:#?} <> {:#?} = {:#?} | {:#?}\n\n", r, mapping.src, intersection, remainder);
                for r in remainder {
                    new_remainders.push(r);
                }
            }
            remainders = new_remainders.clone();
        }
        mapped.extend(remainders.into_iter());
        mapped
    }
}

#[derive(Debug)]
struct Mapping {
    src: Range,
    dst: Range,
}

impl std::str::FromStr for Mapping {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s
            .split(" ")
            .into_iter()
            .map(|i| i.parse::<u64>().map_err(|e| e.into()))
            .collect::<Result<Vec<u64>, AnyError>>()?;
        if vals.len() != 3 {
            bail!("Could not parse mapping line: {}", s);
        }
        Ok(Mapping {
            src: Range {
                start: vals[1],
                end: vals[1] + vals[2],
            },
            dst: Range {
                start: vals[0],
                end: vals[0] + vals[2],
            },
        })
    }
}

pub fn run(args: &CommandFiveArgs) -> AnyResult<u64> {
    let file = File::open(args.file.as_str()).expect("Should have been able to read the file");
    let mut lines = io::BufReader::new(file)
        .lines()
        .filter_map(|s| s.ok())
        .into_iter();
    let mut collections: Vec<MappingCollection> = vec![];
    let mut seeds: Vec<u64> = vec![];
    while let Some(line) = lines.next() {
        if line.starts_with("seeds:") {
            seeds = line[6..]
                .split_whitespace()
                .map(|n| n.trim().parse::<u64>())
                .collect::<Result<Vec<u64>, ParseIntError>>()?;
        } else if line.contains("map") {
            collections.push(MappingCollection::from_lines(&mut lines).unwrap());
        }
    }
    if !args.two {
        let result = seeds
            .iter()
            .map(|&s| {
                return collections
                    .iter()
                    .fold(s, |curr, collection| collection.map(curr));
            })
            .min()
            .unwrap();
        println!("{}", result);
        Ok(result)
    } else {
        let mut ranges: Vec<Range> = vec![];
        let mut seed_iter = seeds.into_iter();
        while let Some(seed) = seed_iter.next() {
            ranges.push(Range {
                start: seed,
                end: seed + seed_iter.next().unwrap() - 1,
            })
        }
        for collection in collections.iter() {
            let mut new_ranges: Vec<Range> = vec![];
            for range in ranges {
                for r in collection.map_range(range) {
                    new_ranges.push(r);
                }
            }
            ranges = new_ranges.clone();
        }
        let result = ranges.iter().map(|r| r.start).min().unwrap();
        println!("{}", result);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
";
        let mut lines = input.split("\n").map(|l| l.to_string()).into_iter();
        let mut collections: Vec<MappingCollection> = vec![];
        while let Some(line) = lines.next() {
            if line.contains("map") {
                collections.push(MappingCollection::from_lines(&mut lines).unwrap());
            }
        }
        assert_eq!(2, collections.len());
        assert_eq!(2, collections[0].mappings.len());
        assert_eq!(3, collections[1].mappings.len());
    }

    #[test]
    fn test_input() {
        let r = self::run(&CommandFiveArgs {
            file: "./inputs/five_test.txt".to_string(),
            two: false,
        });
        assert_eq!(r.unwrap(), 35,);
    }

    #[test]
    fn test_input_part_two() {
        let r = self::run(&CommandFiveArgs {
            file: "./inputs/five_test.txt".to_string(),
            two: true,
        });
        assert_eq!(r.unwrap(), 46,);
    }
}
