pub fn pkcs_7_padding(input: &str, block_size: usize) -> String {
    let padding_size = block_size - input.len() % block_size;
    let padding_char = padding_size as u8 as char;
    let padding_string = padding_char.to_string().repeat(padding_size);
    format!("{}{}", input, padding_string)
}

#[cfg(test)]
mod tests {
    use super::pkcs_7_padding;

    #[test]
    fn challenge_9_test() {
        let result = pkcs_7_padding("YELLOW SUBMARINE", 20);
        let expected = "YELLOW SUBMARINE\x04\x04\x04\x04";

        assert_eq!(result, expected);

        let result = pkcs_7_padding("YELLOW SUBMARINEYELLOW SUBMARINE", 16);
        let expected = format!(
            "{}{}",
            "YELLOW SUBMARINEYELLOW SUBMARINE",
            "\x10".repeat(16)
        );
        assert_eq!(result, expected);
    }
}
