use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    //  810  679   10
    Some(
        input
            .trim()
            .lines()
            .map(|line| {
                let (a, b, c) = line.split_whitespace().collect_tuple().unwrap();
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                let c = c.parse::<usize>().unwrap();
                if a + b > c && b + c > a && a + c > b {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    // part one but with annoying parsing
    let mut acc = vec![];
    let total = input.trim().lines().fold(0, |mut total, line| {
        let (a, b, c) = line.split_whitespace().collect_tuple().unwrap();
        acc.push(a.parse::<usize>().unwrap());
        acc.push(b.parse::<usize>().unwrap());
        acc.push(c.parse::<usize>().unwrap());
        if acc.len() == 9 {
            for n in 0..3 {
                let a = acc[n];
                let b = acc[n + 3];
                let c = acc[n + 6];
                if a + b > c && b + c > a && a + c > b {
                    total += 1;
                }
            }
            acc.clear();
        }
        total
    });
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[rstest]
    #[case(
        r#"101 301 501
    102 302 502
    103 303 503
    201 401 601
    202 402 602
    203 403 603"#,
        Some(6)
    )]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u64>) {
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
