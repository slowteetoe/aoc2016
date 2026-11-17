advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<String> {
    let input = input.trim();

    let mut passwd = [0; 8];
    let mut curr = 0u64;

    let mut n = 0;

    let mut buf = Vec::with_capacity(input.len() + 20); // we'll never need 20 bytes for the curr
    let mut itoa_buf = itoa::Buffer::new();

    loop {
        // more complicated, but gets rid of the allocation I had before with
        //  format!("{input}{curr}").as_bytes()
        buf.clear();
        buf.extend_from_slice(input.as_bytes());
        buf.extend_from_slice(itoa_buf.format(curr).as_bytes());

        let digest = fast_md5::digest(&buf);

        // since we're looking for 5 zeros, we need the high nibble of digest[2] (hex digits are 4 bits)
        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0) == 0 {
            let sixth = digest[2] & 0x0F; // low nibble, 6th char
            passwd[n] = b"0123456789abcdef"[sixth as usize];
            n += 1;
            if n == 8 {
                break;
            }
        }

        curr += 1;
    }

    let passwd = String::from_utf8(passwd.to_vec()).expect("should have worked");
    Some(passwd)
}

pub fn part_two(input: &str) -> Option<String> {
    let input = input.trim();

    let mut passwd = [b'.'; 8];
    let mut curr = 0u64;

    let mut buf = Vec::with_capacity(input.len() + 20); // we'll never need 20 bytes for the curr
    let mut itoa_buf = itoa::Buffer::new();

    loop {
        curr += 1;
        // more complicated, but gets rid of the allocation I had before with
        //  format!("{input}{curr}").as_bytes()
        buf.clear();
        buf.extend_from_slice(input.as_bytes());
        buf.extend_from_slice(itoa_buf.format(curr).as_bytes());

        let digest = fast_md5::digest(&buf);

        // since we're looking for 5 zeros, we need the high nibble of digest[2] (hex digits are 4 bits)
        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0) == 0 {
            let pos = digest[2] & 0x0F; // low nibble, 6th char
            let seventh = digest[3] >> 4; // mask would leave these bits in the wrong place, e.g. 0xa7 & 0xF0 = 1010 0000 = 160
            if pos >= 8 {
                continue;
            }
            if passwd[pos as usize] == b'.' {
                let ch = b"0123456789abcdef"[seventh as usize];
                passwd[pos as usize] = ch;
            }
            if !passwd.contains(&b'.') {
                break;
            }
        }
    }

    let passwd = String::from_utf8(passwd.to_vec()).expect("should have worked");
    Some(passwd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("18f47a30".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("05ace8e3".to_owned()));
    }
}
