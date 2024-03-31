use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use fixed_xor::fixed_xor;
pub mod base64;
pub mod challenge_4;
pub mod challenge_5;
pub mod challenge_6;
pub mod challenge_7;
pub mod fixed_xor;
pub mod single_byte_xor_cipher;
pub mod challenge_8;

/// Remove leading "0x" from hex string (optional) and grouping hex bytes
pub fn hex_decode(hex: &str) -> Vec<u8> {
    hex.trim_start_matches("0x")
        .as_bytes()
        .chunks(2)
        .map(|nibbles| std::str::from_utf8(&nibbles).unwrap())
        .map(|s| u8::from_str_radix(&s, 16).unwrap())
        .collect::<Vec<u8>>()
}

pub fn hex_encode(vec: Vec<u8>) -> String {
    vec.iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn hamming_distance(str1: &[u8], str2: &[u8]) -> u64 {
    fixed_xor(&str1.to_vec(), &str2.to_vec())
        .iter()
        .fold(0, |a, b| a + b.count_ones() as u64)
}

pub fn read_base64_file(path: String) -> Vec<u8> {
    let mut content = String::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            content.push_str(&line);
        }
    }
    ::base64::Engine::decode(&::base64::engine::general_purpose::STANDARD, content).unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::hamming_distance;

    #[test]
    fn hamming_distance_test() {
        let str1 = "this is a test";
        let str2 = "wokka wokka!!!";
        let expected = 37 as u64;
        let result = hamming_distance(str1.as_bytes(), str2.as_bytes());
        assert_eq!(result, expected);
    }
}
