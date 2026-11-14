use std::{collections::BTreeMap, sync::LazyLock};

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(4);

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+(?:-[a-z]+)*)-(\d+)\[([a-z]+)\]$").unwrap());

pub fn parse_checksum(input: &str) -> u32 {
    if let Some(captures) = RE.captures(input) {
        let (_full, [encoded, sector, checksum]) = captures.extract();
        // encoded will be: aaaaa-bbb-z-y-x
        let char_map = encoded
            .chars()
            .fold(BTreeMap::<char, u32>::new(), |mut acc, c| {
                if c != '-' {
                    acc.entry(c).and_modify(|val| *val += 1).or_insert(1);
                }
                acc
            });
        // sorted in reverse order because we're going to pop them off when validating the checksum...
        let mut sorted_tuples = char_map
            .iter()
            .sorted_by(|a, b| Ord::cmp(a.1, b.1).then(Ord::cmp(b.0, a.0)))
            .collect_vec();

        for c in checksum.chars() {
            let next_most_freq = sorted_tuples.pop().unwrap();
            if next_most_freq.0 != &c {
                // invalid checksum
                return 0;
            }
        }
        // valid checksum
        return sector.parse::<u32>().unwrap();
    }
    unreachable!("failed to parse the input row")
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().lines().map(parse_checksum).sum())
}

pub fn decrypt(input: &str) -> (String, u64) {
    let (input, _) = input.splitn(2, "[").collect_tuple().unwrap();
    let (phrase, key) = input.rsplit_once("-").unwrap();
    let key = key.parse::<u64>().unwrap();
    let rot = key.rem_euclid(26) as u8;
    (
        phrase
            .as_bytes()
            .iter()
            .map(|c| match c {
                b'a'..=b'z' => ((c - b'a' + rot) % 26 + b'a') as char,
                b'-' => ' ',
                _ => unreachable!("invalid byte"),
            })
            .join("")
            .to_owned(),
        key,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    // had to print all the decrypted strings out in order to figure out which
    // location we were looking for, only one mentioned the north pole
    input
        .trim()
        .lines()
        .map(decrypt)
        .find(|v| v.0 == "northpole object storage")
        .map(|v| v.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("aaaaa-bbb-z-y-x-123[abxyz]", Some(123))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("qzmt-zixmtkozy-ivhz-343[abxyz]", ("very encrypted name".to_owned(),343))]
    fn test_decrypt(#[case] input: &str, #[case] expected: (String, u64)) {
        let result = decrypt(input);
        assert_eq!(result, expected);
    }
}
