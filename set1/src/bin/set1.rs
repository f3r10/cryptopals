use set1::{base64::hex_to_base64, single_byte_xor_cipher::decipher_hex_message};

fn main() {
    println!("hex_to_base64: {}", hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));

    let msg_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    println!("decipher_message {:?}", decipher_hex_message(msg_hex));
}
