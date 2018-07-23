use crypto::{ buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

// https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs

pub fn ecb_decrypt(cipher_text: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128, key, blockmodes::NoPadding);

    let mut plain_text = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(&cipher_text);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        match decryptor.decrypt(&mut read_buffer, &mut write_buffer, true) {
            Err(_) => return Err("An error occurred"),
            Ok(result) => {
                plain_text.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
                match result {
                    BufferResult::BufferUnderflow => break,
                    BufferResult::BufferOverflow => { }
                }
            }
        }        
    }

    Ok(plain_text)
}

pub fn ecb_encrypt(plain_text: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut encryptor = aes::ecb_encryptor(aes::KeySize::KeySize128, key, blockmodes::NoPadding);

    let mut cipher_text = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(&plain_text);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        match encryptor.encrypt(&mut read_buffer, &mut write_buffer, true) {
            Err(_) => return Err("An error occurred"),
            Ok(result) => {
                cipher_text.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
                match result {
                    BufferResult::BufferUnderflow => break,
                    BufferResult::BufferOverflow => { }
                }
            }
        }        
    }

    Ok(cipher_text)
}

pub fn cbc_decrypt(cipher_text: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut prev_block = iv;
    let mut plain_text = Vec::<u8>::new();

    for chunk in cipher_text.chunks(16) {
        let plain_text_block = match ecb_decrypt(chunk, key) {
            Err(error) => return Err(error),
            Ok(plain_text_bytes) => {
                plain_text_bytes.iter()
                    .zip(prev_block)
                    .map(|(x, y)| x ^ y)
                    .collect::<Vec<u8>>()
            }
        };

        plain_text.extend(plain_text_block);
        prev_block = &chunk;
    }
  
    Ok(plain_text)
}

pub fn cbc_encrypt(plain_text: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut prev_block = iv.to_vec();
    let mut cipher_text = Vec::<u8>::new();

    for chunk in plain_text.chunks(16) {
        let result = chunk.iter()
            .zip(prev_block)
            .map(|(x, y)| x ^ y)
            .collect::<Vec<u8>>();

        let cipher_text_block = match ecb_encrypt(&result, key) {
            Err(error) => return Err(error),
            Ok(cipher_text_bytes) => cipher_text_bytes
        };

        cipher_text.extend(&cipher_text_block);
        prev_block = cipher_text_block;
    }
  
    Ok(cipher_text)
}