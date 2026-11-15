pub mod template;

// Use this file to add helper functions and additional modules.

pub mod day4 {
    use std::{collections::BTreeMap, sync::LazyLock};

    use itertools::Itertools;
    use regex::Regex;

    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^([a-z]+(?:-[a-z]+)*)-(\d+)\[([a-z]+)\]$").unwrap());

    // Stage 1: split a line into (encoded name, sector, checksum).
    pub fn parse_line(input: &str) -> (&str, u32, &str) {
        let caps = RE.captures(input).expect("failed to parse the input row");
        let (_, [encoded, sector, checksum]) = caps.extract();
        (encoded, sector.parse().unwrap(), checksum)
    }

    /// Stage 2: count letters (dashes ignored).
    pub fn count_letters(encoded: &str) -> BTreeMap<char, u32> {
        encoded
            .chars()
            .fold(BTreeMap::<char, u32>::new(), |mut acc, c| {
                if c != '-' {
                    *acc.entry(c).or_insert(0) += 1;
                }
                acc
            })
    }

    /// Stage 2: count letters (dashes ignored).
    pub fn count_letters_arr(encoded: &str) -> [u8; 26] {
        let mut arr = [0; 26];
        for b in encoded.bytes().filter(u8::is_ascii_lowercase) {
            arr[(b - b'a') as usize] += 1;
        }
        arr
    }

    pub fn ranked_letters(counts: [u8; 26]) -> Vec<u8> {
        let mut letters: Vec<(_, _)> = counts
            .iter()
            .enumerate()
            .filter(|&(_, &n)| n > 0)
            .map(|(i, &n)| (b'a' + i as u8, n))
            .collect();

        letters.sort_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)));
        letters.iter().map(|(c, _)| *c).collect()
    }

    pub fn top_five(char_map: BTreeMap<char, u32>) -> Vec<(char, u32)> {
        // sorted in reverse order because we're going to pop them off when validating the checksum...
        char_map
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.1, &b.1).then(Ord::cmp(&b.0, &a.0)))
            .take(5)
            .collect_vec()
    }

    pub fn parse_checksum(input: &str) -> u32 {
        let (encoded, sector, checksum) = parse_line(input);
        let char_map = count_letters_arr(encoded);
        let mut ranked_chars = ranked_letters(char_map);

        for c in checksum.bytes() {
            let next_most_freq = ranked_chars.pop().unwrap();
            if next_most_freq != c {
                // invalid checksum
                return 0;
            }
        }
        // valid checksum
        sector
    }
}
