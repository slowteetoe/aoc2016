use std::collections::BTreeMap;

use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<String> {
    let keypad = BTreeMap::from([
        ((0, 0), "1"),
        ((0, 1), "2"),
        ((0, 2), "3"),
        ((1, 0), "4"),
        ((1, 1), "5"),
        ((1, 2), "6"),
        ((2, 0), "7"),
        ((2, 1), "8"),
        ((2, 2), "9"),
    ]);
    let mut curr = (1, 1); // start at "5"
    let code = input
        .trim()
        .lines()
        .map(|line| {
            let mut terminus = "";
            line.chars().for_each(|c| {
                let delta = match c {
                    'L' => (0, -1),
                    'R' => (0, 1),
                    'U' => (-1, 0),
                    'D' => (1, 0),
                    _ => unreachable!(),
                };
                let next_pos = (curr.0 + delta.0, curr.1 + delta.1);
                // println!(
                //     "At {:?} with instruction {c} so looking {:?} to {:?}",
                //     &curr, &delta, &next_pos
                // );
                if let Some(valid_key) = keypad.get(&next_pos) {
                    curr = next_pos;
                    terminus = *valid_key;
                }
            });
            terminus
        })
        .collect_vec();
    Some(code.join(""))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ULL", Some("1".into()))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<String>) {
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
