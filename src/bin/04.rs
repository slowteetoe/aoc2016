use advent_of_code::day4::parse_checksum;
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().lines().map(parse_checksum).sum())
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
