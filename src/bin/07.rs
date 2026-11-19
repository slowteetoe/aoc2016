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

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
