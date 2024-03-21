use crate::{fixed_xor::fixed_xor, hex_encode};

pub fn repeated_encrypte_xor(message: &str, key: &str) -> String {
    let message_bytes = message.as_bytes();
    let key_bytes_seq = key
        .as_bytes()
        .into_iter()
        .cloned()
        .cycle()
        .take(message_bytes.len())
        .collect::<Vec<u8>>();
    let encryted_xor = fixed_xor(&message_bytes.to_vec(), &key_bytes_seq);
    hex_encode(encryted_xor)
}

#[cfg(test)]
mod tests {
    use crate::challenge_5::repeated_encrypte_xor;

    #[test]
    fn repeated_encrypte_xor_test() {
        let message = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let result = repeated_encrypte_xor(message, key);
        assert_eq!(result, expected);
    }
}
