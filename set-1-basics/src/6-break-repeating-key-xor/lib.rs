use std::f32;
use std::path::Path;
use base64::decode;
use common::{ file, xor_cipher };

pub fn decrypt(path: &Path) -> Result<String, &'static str> {
    if !path.exists() {
        Err("File not found")
    } else {
        let cipher_text = read_file(path);
        let key_length = estimate_key_length(&cipher_text);
        let blocks = transpose(&cipher_text, key_length);

        let key_chars: Vec<char> = blocks.iter()
            .map(|block| {
                calculate_single_character_key(&block)
            })
            .collect();

        let mut plain_bytes = Vec::new();
        cipher_text.iter()
            .enumerate()
            .for_each(|(i, cipher_byte)| {
                let key_byte = key_chars[i % key_length] as u8;
                let plain_byte = cipher_byte ^ key_byte;
                plain_bytes.push(plain_byte)               
            });

        Ok(String::from_utf8(plain_bytes).unwrap())
    }
}

fn read_file(path: &Path) -> Vec<u8> {
    let content = file::read_lines(path)
        .iter()
        .fold(String::new(), |mut content, ref line| {
            content.push_str(line);
            content
        });
    
    decode(&content).unwrap()
}

fn estimate_key_length(cipher_text: &[u8]) -> usize {
    let mut best_key_length = 0;
    let mut minimum_distance = f32::INFINITY;

    (4..=40).for_each(|key_length| {
        let mut count = 0;
        let mut distance = 0;
        let limit = cipher_text.len() / key_length;

        for i in 1..(limit - 1) {
            count += 1;
            let first = &cipher_text[(key_length * (i - 1))..(key_length * i)];
            let second = &cipher_text[(key_length * i)..((i + 1) * key_length)];
            distance += hamming_distance(&first, &second);
        }   

        let normalized_distance = distance as f32 / (count as f32) / (key_length as f32);
        if normalized_distance < minimum_distance {
            minimum_distance = normalized_distance;
            best_key_length = key_length;
        }     
    });

    best_key_length
}

fn transpose(input: &[u8], key_length: usize) -> Vec<Vec<u8>> {
    let mut blocks = Vec::new();
    for i in 0..key_length {
        let mut block = Vec::new();    

        let mut j = i;
        while j < input.len() {
            block.push(input[j]);
            j += key_length;
        }

        blocks.push(block);
    }
    blocks
}

fn calculate_single_character_key(input: &[u8]) -> char {
    let mut key = 'a';
    let mut maximum_score = 0.0;

    xor_cipher::try_decrypt_single_character_xor(input).iter()
        .for_each(|res| {
            if res.score > maximum_score {
                key = res.key;
                maximum_score = res.score;
            }
        });

    key
}

fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b)
        .map(|(a_byte, b_byte)| a_byte ^ b_byte)
        .fold(0, |distance, mut byte| {
            let mut delta = 0;
            while byte != 0 {
                if byte % 2 == 1 {
                    delta += 1;
                }
                byte >>= 1;
            }
            distance + delta
        })        
}