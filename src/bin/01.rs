use std::collections::BTreeSet;

advent_of_code::solution!(1);

#[derive(Debug)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    pub fn next(self, turn: &str) -> Direction {
        match turn {
            "R" => match self {
                Direction::N => Self::E,
                Direction::S => Self::W,
                Direction::E => Self::S,
                Direction::W => Self::N,
            },
            "L" => match self {
                Direction::N => Self::W,
                Direction::S => Self::E,
                Direction::E => Self::N,
                Direction::W => Self::S,
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Santa {
    pub heading: Direction,
    pub position: (i64, i64),
    memory: BTreeSet<(i64, i64)>,
}

impl Santa {
    pub fn new() -> Self {
        let initial = (0, 0);
        let mut memory = BTreeSet::new();
        memory.insert(initial);
        Santa {
            position: initial,
            heading: Direction::N,
            memory,
        }
    }
    pub fn go(&mut self, direction: &str) {
        let magnitude = direction[1..].parse::<i64>().unwrap();
        match &direction[0..1] {
            "R" => match self.heading {
                Direction::N => {
                    self.heading = Direction::E;
                    self.position.0 += magnitude
                }
                Direction::S => {
                    self.heading = Direction::W;
                    self.position.0 -= magnitude
                }
                Direction::E => {
                    self.heading = Direction::S;
                    self.position.1 += magnitude;
                }
                Direction::W => {
                    self.heading = Direction::N;
                    self.position.1 -= magnitude;
                }
            },
            "L" => match self.heading {
                Direction::N => {
                    self.heading = Direction::W;
                    self.position.0 -= magnitude
                }
                Direction::S => {
                    self.heading = Direction::E;
                    self.position.0 += magnitude
                }
                Direction::E => {
                    self.heading = Direction::N;
                    self.position.1 -= magnitude;
                }
                Direction::W => {
                    self.heading = Direction::S;
                    self.position.1 += magnitude;
                }
            },
            _ => unreachable!(),
        };
        // dbg!(&self);
    }

    pub fn go_and_watch_for_dups(&mut self, direction: &str) -> Option<u64> {
        // ugh, implementation of p1 made this nasty
        let magnitude = direction[1..].parse::<i64>().unwrap();
        let step = match &direction[0..1] {
            "R" => match self.heading {
                Direction::N => {
                    self.heading = Direction::E;
                    (1, 0)
                }
                Direction::S => {
                    self.heading = Direction::W;
                    (-1, 0)
                }
                Direction::E => {
                    self.heading = Direction::S;
                    (0, 1)
                }
                Direction::W => {
                    self.heading = Direction::N;
                    (0, -1)
                }
            },
            "L" => match self.heading {
                Direction::N => {
                    self.heading = Direction::W;
                    (-1, 0)
                }
                Direction::S => {
                    self.heading = Direction::E;
                    (1, 0)
                }
                Direction::E => {
                    self.heading = Direction::N;
                    (0, -1)
                }
                Direction::W => {
                    self.heading = Direction::S;
                    (0, 1)
                }
            },
            _ => unreachable!(),
        };

        for _ in 0..magnitude {
            // dbg!(&self.memory);
            self.position.0 += step.0;
            self.position.1 += step.1;
            if self.memory.contains(&self.position) {
                // println!("re-visited {:?}", &self.position);
                return Some(self.final_distance());
            } else {
                self.memory.insert(self.position);
            }
        }
        None
    }

    pub fn final_distance(&self) -> u64 {
        self.position.0.abs().cast_unsigned() + self.position.1.abs().cast_unsigned()
    }
}

impl Default for Santa {
    fn default() -> Self {
        Self::new()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut santa = Santa::new();
    input.trim().split(", ").for_each(|d| {
        santa.go(d);
    });
    // dbg!(&santa);
    Some(santa.final_distance())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut santa = Santa::new();
    for d in input.trim().split(", ") {
        if let Some(final_distance) = santa.go_and_watch_for_dups(d) {
            return Some(final_distance);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("R2, L3", 5)]
    #[case("R2, R2, R2", 2)]
    #[case("R5, L5, R5, R3", 12)]
    fn test_part_one(#[case] input: &str, #[case] expected: u64) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("R8, R4, R4, R8", 4)]
    fn test_part_two(#[case] input: &str, #[case] expected: u64) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
