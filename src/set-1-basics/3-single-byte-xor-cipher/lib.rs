use std::collections::HashMap;

pub fn decrypt(input: &str) -> Result<String, &'static str> {
    if !input.is_ascii() {
        Err("Input string is not valid ASCII")
    } else {
        let numbers = to_numbers(input);

        let mut maximum_score = 0.0;
        let mut secret_message = String::new();
        (0..=255).map(|ch| xor(&numbers, ch))
            .map(|xor| to_string(&xor))
            .for_each(|msg| {
                let value = score(&msg);
                if value > maximum_score {
                    maximum_score = value;
                    secret_message = msg;
                }
            });
        
        Ok(secret_message)
    }    
}

pub fn to_numbers(input: &str) -> Vec<u8> {
    input.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chars| hex_to_number(&to_string(&chars)))
        .collect()
}

fn hex_to_number(input: &str) -> u8 {
    u8::from_str_radix(input, 16).unwrap()
}

fn to_string(input: &[char]) -> String {
    input.iter().collect::<String>()
}

fn xor(input: &[u8], key: u8) -> Vec<char> {
    input.iter()
        .map(|x| (x ^ key) as char)
        .collect()
}

pub fn score(input: &str) -> f32 {
    let frequencies: HashMap<char, f32> =
        [
            ('a', 0.0651738),
            ('b', 0.0124248),
            ('c', 0.0217339),
            ('d', 0.0349835),
            ('e', 0.1041442),
            ('f', 0.0197881),
            ('g', 0.0158610),
            ('h', 0.0492888),
            ('i', 0.0558094),
            ('j', 0.0009033),
            ('k', 0.0050529),
            ('l', 0.0331490),
            ('m', 0.0202124),
            ('n', 0.0564513),
            ('o', 0.0596302),
            ('p', 0.0137645),
            ('q', 0.0008606),
            ('r', 0.0497563),
            ('s', 0.0515760),
            ('t', 0.0729357),
            ('u', 0.0225134),
            ('v', 0.0082903),
            ('w', 0.0171272),
            ('x', 0.0013692),
            ('y', 0.0145984),
            ('z', 0.0007836),
            (' ', 0.1918182)
        ]
        .iter().cloned().collect();
    
    input.chars()
        .map(|ch| {
            match frequencies.get(&ch) {
                None => 0.0,
                Some(&freq) => freq
            }
        })
        .sum()
}