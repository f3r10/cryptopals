use crate::{hex_decode, read_lines};

pub fn detect_aes_in_ecb_mode(path: String) -> String {
    let mut result = String::new();
    let mut max_identical_blocks = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            let hex_decoded = hex_decode(&line);
            let mut blocks = hex_decoded.chunks_exact(16).collect::<Vec<_>>();
            let len = blocks.len();
            blocks.sort();
            blocks.dedup();
            let repeated_blocks = len - blocks.len();
            if repeated_blocks > max_identical_blocks {
                max_identical_blocks = repeated_blocks;
                result = line;
            }
        }
    };
    result
}

#[cfg(test)]
mod test {
    use crate::challenge_8::detect_aes_in_ecb_mode;

    #[test]
    fn challenge_8_test() {
        let result = detect_aes_in_ecb_mode("../files/8.txt".to_string());
        let expected = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5\
             d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649\
             af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c\
             7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6\
             aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd2\
             83d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c\
             58386b06fba186a";
        assert_eq!(result, expected);
    }
}
