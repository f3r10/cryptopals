use crate::{
    fixed_xor::fixed_xor, hex_decode, read_lines, single_byte_xor_cipher::break_single_byte_xor,
};

pub fn decipher_hidden_line(path: String) -> String {
    let mut message = String::new();
    let mut best_score: f64 = f64::MIN;
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            let hex_decode = hex_decode(&line);
            let (key, score) = break_single_byte_xor(&hex_decode);
            if score > best_score {
                best_score = score;
                message =
                    String::from_utf8_lossy(&fixed_xor(&hex_decode, &vec![key; hex_decode.len()]))
                        .to_string();
            }
        }
    }
    message
}

#[cfg(test)]
mod tests {
    use crate::challenge_4::decipher_hidden_line;

    #[test]
    fn challenge_4_test() {
        let expected = "Now that the party is jumping\n";
        let result = decipher_hidden_line("../files/4.txt".to_string());
        assert_eq!(result, expected);
    }
}
