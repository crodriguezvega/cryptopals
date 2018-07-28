use itertools::Itertools;
use rand::{ thread_rng, Rng };
use shared::{ aes, pkcs7 };

#[derive(Debug, PartialEq, Eq)]
pub enum AesMode {
    ECB,
    CBC        
}

pub fn encryption_oracle(plain_text: &[u8]) -> Result<(AesMode, Vec<u8>), &'static str> {
    let mut rng = thread_rng();
    let switch: bool = rng.gen();
    let key: [u8; 16] = rng.gen();
    let iv: [u8; 16] = rng.gen();

    let mut prefix = generate_random_vector();
    let mut suffix = generate_random_vector();
    
    let mut input = Vec::new();
    input.append(&mut prefix);
    input.extend(plain_text);
    input.append(&mut suffix);

    let padded_input = pkcs7::pad(&input, 16);

    if switch {
        match aes::ecb_encrypt(&padded_input, &key) {
            Err(error) => Err(error),
            Ok(cipher_text) => Ok((AesMode::ECB, cipher_text))
        }
    } else {
        match aes::cbc_encrypt(&padded_input, &key, &iv){
            Err(error) => Err(error),
            Ok(cipher_text) => Ok((AesMode::CBC, cipher_text))
        } 
    }
}

pub fn detect_aes_mode(cipher_text: &[u8]) -> AesMode {
    let result = cipher_text.chunks(16)
        .combinations(2)
        .fold(0, |acc, combination| {
            if combination[0] == combination[1] {
                acc + 1
            } else {
                acc
            }
        });

    if result > 0 {
        AesMode::ECB
    } else {
        AesMode::CBC
    }
}

fn generate_random_vector() -> Vec<u8> {
    let mut rng = thread_rng();
    let length = rng.gen_range(5, 11);
    let mut vector = vec!(0; length);
    rng.fill(&mut vector[..]);
    vector
}