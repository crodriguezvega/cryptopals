pub fn pad(input: &[u8], block_length: usize) -> Vec<u8>{
    let length = input.len();    
    let mut cloned = input.to_vec();

    let mut padding = 0;
    if block_length > length {
        padding = block_length - length;
    }
    if block_length < length {
        padding = block_length - length % block_length;       
    }

    if padding > 0 {
        (1..=padding).for_each(|_| cloned.push(padding as u8));
    }

    cloned
}