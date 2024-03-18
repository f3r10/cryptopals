use std::iter::zip;

pub fn fixed_xor(input1_bytes: &Vec<u8>, input2_bytes: &Vec<u8>) -> Vec<u8> {
    if input1_bytes.len() != input2_bytes.len() {
        panic!("inputs should be the same size")
    }

    let result = zip(input1_bytes, input2_bytes)
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>();
    return result;
}

#[cfg(test)]
mod tests {
    use crate::{fixed_xor::fixed_xor, hex_decode, hex_encode};

    #[test]
    fn fixed_xor_test() {
        let expected = "746865206b696420646f6e277420706c6179";
        let input1 = "1c0111001f010100061a024b53535009181c";
        let input2 = "686974207468652062756c6c277320657965";
        let result = fixed_xor(&hex_decode(input1), &hex_decode(input2));
        assert_eq!(expected, hex_encode(result));
    }
}
