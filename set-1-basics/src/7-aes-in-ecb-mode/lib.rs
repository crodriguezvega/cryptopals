use std::path::Path;
use crypto::{ buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use common::file;

pub fn decrypt(path: &Path, key: &str) -> Result<String, &'static str> {
    if !path.exists() {
        Err("File not found")
    } else if !key.is_ascii() {
        Err("Key is not valid ASCII")
    } else {
        let cipher_text = file::read_file(path);
        let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128, key.as_bytes(), blockmodes::NoPadding);

        let mut plain_text = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(&cipher_text);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap(); 
            plain_text.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }

        match String::from_utf8(plain_text) {
            Err(_) => Err("Plain text is not valid UTF8"),
            Ok(plain_text) => Ok(plain_text)
        }
    }
}