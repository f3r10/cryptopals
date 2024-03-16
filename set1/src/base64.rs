pub fn hex_to_base64(hex_str: &str) -> String {
    // Remove leading "0x" from hex string (optional) and grouping hex bytes
    let hex_bytes = hex_str
        .trim_start_matches("0x")
        .as_bytes()
        .chunks(2)
        .map(|nibbles| std::str::from_utf8(&nibbles).unwrap())
        .map(|s| u8::from_str_radix(&s, 16).unwrap())
        .collect::<Vec<u8>>();

    // println!("hex_str_bytes {:?}", hex_str.trim_start_matches("0x").as_bytes());
    // println!("hex_bytes {:?}", hex_bytes);

    let mut base64 = String::with_capacity(4 * hex_bytes.len() / 3);
    for block in hex_bytes.chunks(3) {
        for b64_char in block_to_base64(block) {
            base64.push(b64_char);
        }
    }

    // padding
    if hex_bytes.len() % 3 >= 1 {
        base64.pop();
        if hex_bytes.len() % 3 == 1 {
            base64.pop();
            base64.push('=');
        }
        base64.push('=');
    }
    base64
}

fn block_to_base64(block: &[u8]) -> Vec<char> {
    let mut b64_chars = Vec::with_capacity(4);

    let (a, b, c) = match block.len() {
        3 => (block[0], block[1], block[2]),
        2 => (block[0], block[1], 0),
        1 => (block[0], 0, 0),
        _ => return vec![],
    };

    // first 6 bits of a
    b64_chars.push(u8_to_base64(a >> 2));

    // last 2 bits of `a` and 4 first bits of `b`
    b64_chars.push(u8_to_base64(a % 4 * 16 + (b >> 4)));

    // last 4 bits of `b` and 2 first bits of `c`
    b64_chars.push(u8_to_base64(b % 16 * 4 + (c >> 6)));

    // last 6 bits of c
    b64_chars.push(u8_to_base64(c & 0x3f));

    b64_chars
}

fn u8_to_base64(u: u8) -> char {
    match u {
        0..=25 => (b'A' + u) as char,
        26..=51 => (b'a' + (u - 26)) as char,
        52..=61 => (b'0' + (u - 52)) as char,
        62 => '=',
        63 => '/',
        _ => panic!("byte exceeded range {}", u),
    }
}


#[cfg(test)]
mod tests {
    use crate::base64::hex_to_base64;

    #[test]
    fn hex_to_base64_test() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let base64 = hex_to_base64(input);
        assert_eq!(expected_base64, &base64);
    }
}
