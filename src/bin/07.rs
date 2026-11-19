use std::collections::BTreeSet;

use itertools::Itertools;

advent_of_code::solution!(7);

fn is_abba(input: &str) -> bool {
    let mut it = input.chars();
    let (Some(mut a), Some(mut b), Some(mut c)) = (it.next(), it.next(), it.next()) else {
        // we're at the end of our windows
        return false;
    };
    for d in it {
        if a == d && b == c && a != b {
            return true;
        }
        a = b;
        b = c;
        c = d;
    }
    false
}

// maybe there can be more than one per segment?
fn is_aba(input: &str) -> Option<Vec<String>> {
    let mut it = input.chars();
    let mut matches = vec![];
    let (Some(mut a), Some(mut b)) = (it.next(), it.next()) else {
        // only 2 chars left, can't have an a-b-a triplet
        return None;
    };
    for c in it {
        if a == c && a != b {
            // return Some(format!("{a}{b}{c}"));
            matches.push(format!("{a}{b}{c}"));
        }
        a = b;
        b = c;
    }
    if matches.is_empty() {
        None
    } else {
        Some(matches)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let valid_lines = input
        .trim()
        .lines()
        .map(|line| {
            // the input doesn't match the examples; there can be many hypernet sequences, not just one
            let mut counts = (0, 0);
            for (i, segment) in line.split(['[', ']']).enumerate() {
                if is_abba(segment) {
                    if i % 2 == 0 {
                        // outside brackets
                        counts.0 += 1;
                    } else {
                        // inside brackets
                        counts.1 += 1;
                    }
                }
            }
            counts.0 > 0 && counts.1 == 0
        })
        .filter(|valid| *valid)
        .count() as u64;

    Some(valid_lines)
}

#[derive(Default)]
struct SSL {
    pub aba: BTreeSet<String>,
    pub bab: BTreeSet<String>,
}

impl SSL {
    pub fn supported(&self) -> bool {
        self.aba.iter().any(|aba| {
            // we can do this since it's ascii
            let a = &aba[..1];
            let b = &aba[1..2];
            self.bab.contains(&format!("{b}{a}{b}"))
        })
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut sets = SSL::default();
            for (i, segment) in line.split(['[', ']']).enumerate() {
                if let Some(matches) = is_aba(segment) {
                    if i % 2 == 0 {
                        // outside brackets
                        for s in matches {
                            sets.aba.insert(s);
                        }
                    } else {
                        for s in matches {
                            sets.bab.insert(s);
                        }
                    }
                }
            }
            if sets.supported() { 1 } else { 0 }
        })
        .sum1()
    // 147 too low
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abba[mnop]qrst", Some(1))]
    #[case("abcd[bddb]xyyx", Some(0))]
    #[case("aaaa[qwer]tyui", Some(0))]
    #[case("ioxxoj[asdfgh]zxcvbn", Some(1))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u64>) {
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("aba", Some(vec!["aba".to_owned()]))]
    #[case("aaa", None)]
    #[case("aaaacaaaa", Some(vec!["aca".to_owned()]))]
    fn test_aba_match(#[case] input: &str, #[case] expected: Option<Vec<String>>) {
        let result = is_aba(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_supported_matching() {
        let mut s = SSL::default();
        s.aba.insert("xyx".to_owned());
        s.bab.insert("yxy".to_owned());
        assert!(s.supported());
    }

    #[test]
    fn test_supported_unmatched() {
        let mut s = SSL::default();
        s.aba.insert("xyx".to_owned());
        s.bab.insert("yay".to_owned());
        assert!(s.supported().not());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
