advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<String> {
    let mut maps = [[0u8; 26]; 8]; // 8 columns x 26 chars
    input.trim().lines().for_each(|line| {
        line.chars().enumerate().for_each(|(n, ch)| {
            let pos = ch as u8 - b'a';
            maps[n][pos as usize] += 1;
        });
    });

    let word: String = maps
        .iter()
        .map(|counts| {
            let (idx, _) = counts
                .iter()
                .enumerate()
                .max_by_key(|&(_, &count)| count)
                .unwrap();
            (b'a' + idx as u8) as char
        })
        .collect();
    Some(word)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut maps = [[0u8; 26]; 8]; // 8 columns x 26 chars
    input.trim().lines().for_each(|line| {
        line.chars().enumerate().for_each(|(n, ch)| {
            let pos = ch as u8 - b'a';
            maps[n][pos as usize] += 1;
        });
    });

    // same as part 1, but this time we want the least common char
    let word: String = maps
        .iter()
        .map(|counts| {
            let (idx, _) = counts
                .iter()
                .enumerate()
                .filter(|&(_, &count)| count > 0)
                .min_by_key(|&(_, &count)| count)
                .unwrap();
            (b'a' + idx as u8) as char
        })
        .collect();
    Some(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // there are 8 chars in the actual input but only 6 in the example
        let result = &result.unwrap()[0..6];
        assert_eq!(result, "easter".to_owned());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
