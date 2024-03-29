use crate::{
    fixed_xor::dynamic_xor,
    hamming_distance,
    single_byte_xor_cipher::{break_single_byte_xor, calc_letter_freq_score},
};

pub fn get_normalized_hamming_distance(input: &[u8], k_size: usize) -> f32 {
    let mut distance = 0f32;
    let mut counter = 0f32;
    input
        .chunks_exact((k_size * 2) as usize)
        .for_each(|blocks| {
            let block_1 = &blocks[0..k_size];
            let block_2 = &blocks[k_size..];
            distance += hamming_distance(block_1, block_2) as f32 / (k_size as f32);
            counter += 1f32;
        });
    distance / counter
}

pub fn transpose_blocks(input: &[u8], k_size: usize) -> Vec<Vec<u8>> {
    let mut transposed_blocks = (0..k_size).map(|_| Vec::new()).collect::<Vec<Vec<u8>>>();
    for block in input.chunks(k_size) {
        for (&b, bt) in block.iter().zip(transposed_blocks.iter_mut()) {
            bt.push(b)
        }
    }
    transposed_blocks
}

pub fn solve_each_block(transposed_blocks: Vec<Vec<u8>>) -> Vec<u8> {
    transposed_blocks
        .iter()
        .map(|block| {
            let (key, _) = break_single_byte_xor(block);
            key
        })
        .collect::<Vec<_>>()
}

pub fn guessed_keysize(input: &[u8]) -> Vec<usize> {
    let mut distances = (2..41)
        .map(|keysize| (keysize, get_normalized_hamming_distance(input, keysize)))
        .collect::<Vec<(usize, f32)>>();

    distances.sort_by(|&(_, b1), &(_, b2)| b1.total_cmp(&b2));
    distances
        .iter()
        .take(3)
        .map(|x| x.0)
        .collect::<Vec<usize>>()
}

pub fn break_xor_for_keysize(input: &[u8], k_size: usize) -> Vec<u8> {
    transpose_blocks(input, k_size)
        .iter()
        .map(|b| break_single_byte_xor(b))
        .map(|r| r.0)
        .collect::<Vec<u8>>()
}

pub fn break_multiple_xor(input: &[u8]) -> Vec<u8> {
    guessed_keysize(input)
        .iter()
        .map(|&k_size| break_xor_for_keysize(input, k_size))
        .max_by_key(|key| calc_letter_freq_score(&dynamic_xor(&input.to_vec(), &key)) as u32)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{challenge_6::break_multiple_xor, read_base64_file};

    #[test]
    fn challenge_6_test() {
        let input = read_base64_file("../files/6.txt".to_string());
        let expected = b"Terminator X: Bring the noise";
        let result = break_multiple_xor(&input);
        assert_eq!(result, expected);
    }
}
