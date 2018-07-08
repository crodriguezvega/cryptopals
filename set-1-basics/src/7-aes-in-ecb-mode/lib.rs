use crypto::{ buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

pub fn decrypt(cipher_text: &[u8], key: &str) -> Result<String, &'static str> {
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