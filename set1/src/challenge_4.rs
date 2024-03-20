use crate::{read_lines, single_byte_xor_cipher::decipher_message};

pub fn decipher_hidden_line(path: String) -> String {
    let mut message = String::new();
    let mut best_score: f64 = f64::MIN;
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            let (decrypted_line, score) = decipher_message(&line);
            if score > best_score {
                best_score = score;
                message = String::from(decrypted_line);
            }
        }
    }
    message
}
