pub fn pad(plain_text: &[u8], block_length: usize) -> Vec<u8> {
    let padding = block_length - plain_text.len(); 
    let mut cloned = plain_text.to_vec();

    if padding > 0 {
        (1..=padding).for_each(|_| cloned.push(padding as u8));
    }

    cloned
}