pub mod base64;
pub mod fixed_xor;

/// Remove leading "0x" from hex string (optional) and grouping hex bytes
pub fn hex_decode(hex: &str) -> Vec<u8> {
    hex.trim_start_matches("0x")
        .as_bytes()
        .chunks(2)
        .map(|nibbles| std::str::from_utf8(&nibbles).unwrap())
        .map(|s| u8::from_str_radix(&s, 16).unwrap())
        .collect::<Vec<u8>>()
}
