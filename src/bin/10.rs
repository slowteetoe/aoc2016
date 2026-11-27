use std::collections::{BTreeMap, VecDeque};

use anyhow::{Result, anyhow};
use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<String> {
    let defs = input.trim().lines().map(parse_line).collect_vec();
    let mut bots = BTreeMap::new();
    let mut values = VecDeque::new();
    for result in defs {
        match result.unwrap() {
            Definition::Bot { name, low, high } => {
                bots.insert(
                    name.clone(),
                    Bot {
                        name,
                        low_target: low,
                        low_value: None,
                        high_target: high,
                        high_value: None,
                    },
                );
            }
            Definition::Value { n, destination } => {
                values.push_back((destination, n));
            }
        }
    }
    let mut winner = None;
    while let Some((k, v)) = values.pop_front()
        && winner.is_none()
    {
        let mut assignment = None;
        if let Some(bot) = bots.get_mut(&k) {
            assignment = bot.accept(v);
        }
        if let Some((curr_bot, (low_target, low_value, high_target, high_value))) = assignment {
            // println!("{curr_bot} compared {low_value} and {high_value}");

            // from the problem description, we're looking for the bot that compared these two values
            if low_value == 17 && high_value == 61 {
                winner = Some(curr_bot);
            }
            if let Some(lowbot) = bots.get_mut(&low_target) {
                let out = lowbot.accept(low_value);
                if let Some((_, (low_target, low_value, high_target, high_value))) = out {
                    values.push_back((low_target, low_value));
                    values.push_back((high_target, high_value));
                }
            }
            if let Some(highbot) = bots.get_mut(&high_target) {
                let out = highbot.accept(high_value);
                if let Some((_, (low_target, low_value, high_target, high_value))) = out {
                    values.push_back((low_target, low_value));
                    values.push_back((high_target, high_value));
                }
            }
        }
    }
    winner
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Debug, Clone)]
pub enum Definition {
    Bot {
        name: String,
        low: String,
        high: String,
    },
    Value {
        n: u32,
        destination: String,
    },
}

#[derive(Debug, Clone)]
struct Bot {
    name: String,
    low_target: String,
    low_value: Option<u32>,
    high_target: String,
    high_value: Option<u32>,
}

impl Bot {
    pub fn accept(&mut self, val: u32) -> Option<(String, (String, u32, String, u32))> {
        if let Some(low) = self.low_value {
            self.low_value = Some(val.min(low));
            self.high_value = Some(val.max(low));
            Some((
                self.name.clone(),
                (
                    self.low_target.clone(),
                    self.low_value.unwrap(),
                    self.high_target.clone(),
                    self.high_value.unwrap(),
                ),
            ))
        } else {
            self.low_value = Some(val);
            None
        }
    }
}

pub fn parse_line(input: &str) -> Result<Definition> {
    let input = input.trim();
    if let Some((lhs, rhs)) = input.split_once(" gives low to ") {
        // bot 1 gives low to output 1 and high to bot 0
        // bot 0 gives low to output 2 and high to output 0
        let (low, high) = rhs.split_once(" and high to ").unwrap();
        return Ok(Definition::Bot {
            name: lhs.to_owned(),
            low: low.to_owned(),
            high: high.to_owned(),
        });
    } else {
        // value 2 goes to bot 2
        if let Some((lhs, rhs)) = input.trim().split_once(" goes to ") {
            let value = lhs.split_whitespace().nth(1).unwrap();
            let destination = rhs;
            return Ok(Definition::Value {
                n: value.parse().unwrap(),
                destination: destination.to_owned(),
            });
        }
    }
    Err(anyhow!("oops"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
