use std::iter::zip;

use crate::hex_decode;

pub fn fixed_xor(input1: &str, input2: &str) -> String  {
    if input1.len() != input2.len() {
        panic!("inputs should be the same size")
    }
    let input1_bytes = hex_decode(input1);

    let input2_bytes = hex_decode(input2);

    let result = 
        zip(input1_bytes, input2_bytes).map(|(a, b)| a ^ b)
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("");
    return result
    
}

#[cfg(test)]
mod tests {
    use crate::fixed_xor::fixed_xor;


    #[test]
    fn fixed_xor_test() {
        let expected = "746865206b696420646f6e277420706c6179";
        let input1 = "1c0111001f010100061a024b53535009181c";
        let input2 = "686974207468652062756c6c277320657965";
        let result  = fixed_xor(input1, input2);
        assert_eq!(expected, result);
    }
}
