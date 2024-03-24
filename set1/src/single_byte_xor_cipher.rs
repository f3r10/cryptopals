use crate::{fixed_xor::fixed_xor, hex_decode};

//ref: https://crypto.stackexchange.com/questions/30209/developing-algorithm-for-detecting-plain-text-via-frequency-analysis
const LETTER_FREQ: [f64; 27] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 0.19181, // V-Z & space char
];

pub fn calc_letter_freq_score(s: &[u8]) -> f64 {
    let mut counts = vec![0_u32; 27];
    let mut score: f64 = 0_f64;

    s.iter().for_each(|c| match c {
        b'a'..=b'z' => {
            counts[*c as usize - 97] += 1;
        }
        b'A'..=b'Z' => {
            counts[*c as usize - 65] += 1;
        }
        b' ' => counts[26] += 1,
        _ => {}
    });

    for i in 0..27 {
        score += (counts[i] as f64) * LETTER_FREQ[i];
    }

    score
}

pub fn break_single_byte_xor(input: &[u8]) -> (u8, f64) {
    let mut key = 0;
    let mut best_score = f64::MIN;
    for c in 0..=255_u8 {
        let key_bytes = vec![c; input.len()];
        let msg_bytes = fixed_xor(&input.to_vec(), &key_bytes);
        let score = calc_letter_freq_score(&msg_bytes);
        if score > best_score {
            best_score = score;
            key = c
        }
    }

    (key, best_score)
}

pub fn decipher_hex_message(input: &str) -> String {
    let hex_decode = hex_decode(input);
    let (key, _) = break_single_byte_xor(&hex_decode);
    String::from_utf8_lossy(&fixed_xor(&hex_decode, &vec![key; hex_decode.len()])).to_string()
}

#[cfg(test)]
mod test {
    use crate::single_byte_xor_cipher::decipher_hex_message;

    #[test]
    fn challenge_3_test() {
        let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let expected = "Cooking MC's like a pound of bacon";
        let result = decipher_hex_message(hex);
        assert_eq!(result, expected);
    }
}
