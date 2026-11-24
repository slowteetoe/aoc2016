advent_of_code::solution!(9);

fn process_line(input: &str) -> u64 {
    if !input.contains('(') {
        return input.len() as u64;
    }
    let mut letter_count = 0u64;
    let mut instr = None;
    let buf = input.as_bytes();
    let mut idx = 0;
    let max = buf.len();
    loop {
        let b = buf[idx];
        match b {
            b'(' => {
                if instr.clone().is_some() {
                    panic!("misplaced ((")
                }
                instr = Some("".to_owned());
                idx += 1;
            }
            b')' => {
                if let Some(i) = instr.take() {
                    let (lhs, rhs) = i.split_once("x").expect("should have had AxB");
                    let repeat = lhs.parse::<usize>().expect("needed a value for repeat");
                    let times = rhs
                        .parse::<usize>()
                        .expect("needed a value for times to repeat");
                    let start = idx + 1;
                    let end = (start + repeat).min(max); // clamp if data runs short
                    letter_count += ((end - start) as u64) * times as u64;
                    idx = end;
                } else {
                    letter_count += 1;
                    idx += 1;
                }
            }
            ch => {
                if let Some(s) = instr.as_mut() {
                    s.push(ch as char);
                } else {
                    letter_count += 1;
                }
                idx += 1;
            }
        }
        if idx >= max {
            break;
        }
    }
    letter_count
}
fn process_line_p2(input: &str) -> u64 {
    let buf = input.as_bytes();
    let mut idx = 0;
    let mut total = 0u64;

    while idx < buf.len() {
        if buf[idx] == b'(' {
            let close = idx
                + buf[idx..]
                    .iter()
                    .position(|&b| b == b')')
                    .expect("unterminated marker");
            let (lhs, rhs) = input[idx + 1..close]
                .split_once('x')
                .expect("should have had AxB");
            let repeat: usize = lhs.parse().expect("bad repeat");
            let times: u64 = rhs.parse().expect("bad times");

            let start = close + 1;
            let end = (start + repeat).min(buf.len());

            // the only change from part 1: recurse instead of `(end - start) as u64`
            total += process_line_p2(&input[start..end]) * times;
            idx = end;
        } else {
            total += 1;
            idx += 1;
        }
    }
    total
}

pub fn part_one(input: &str) -> Option<u64> {
    input.trim().lines().map(process_line).reduce(|a, b| a + b)
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .trim()
        .lines()
        .map(process_line_p2)
        .reduce(|a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ADVENT", 6)]
    #[case("A(1x5)BC", 7)]
    #[case("(3x3)XYZ", 9)]
    #[case("A(2x2)BCD(2x2)EFG", 11)]
    #[case("(6x1)(1x3)A", 6)]
    #[case("X(8x2)(3x3)ABCY", 18)]
    fn test_process_line(#[case] input: &str, #[case] expected: u64) {
        let result = process_line(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
