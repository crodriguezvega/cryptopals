use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use common::xor_cipher;

pub fn decrypt(path: &Path) -> Result<String, &'static str> {
    if !path.exists() {
        Err("File not found")
    } else {
        let mut maximum_score = 0.0;
        let mut secret_message = String::new();
        read_lines(path).iter()
            .flat_map(|line| xor_cipher::try_decrypt_single_character_xor(&line))
            .for_each(|res| {
                if res.score > maximum_score {
                    maximum_score = res.score;
                    secret_message = res.secret_message.clone();
                }
            });

        Ok(secret_message)
    }
}

fn read_lines(path: &Path) -> Vec<String> {
    let file_handler = File::open(path).unwrap();
    let file = BufReader::new(&file_handler);
    
    file.lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.is_ascii())
        .collect()
}