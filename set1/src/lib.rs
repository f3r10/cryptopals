use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub mod base64;
pub mod challenge_4;
pub mod challenge_5;
pub mod fixed_xor;
pub mod single_byte_xor_cipher;

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
