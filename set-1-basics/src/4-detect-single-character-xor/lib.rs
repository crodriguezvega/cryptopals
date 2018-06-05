use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use common::mappings;
use common::xor_cipher;

pub fn decrypt(path: &Path) -> Result<String, &'static str> {
    if !path.exists() {
        Err("File not found")
    } else {
        let lines = read_lines(path);

        let mut maximum_score = 0.0;
        let mut secret_message = String::new();
        for line in &lines {
            let numbers = mappings::hex_string_to_numbers(line);

            (0..=255).map(|ch| xor_cipher::xor(&numbers, ch))
                .map(|xor| mappings::chars_to_string(&xor))
                .for_each(|msg| {
                    let value = xor_cipher::score(&msg);
                    if value > maximum_score {
                        maximum_score = value;
                        secret_message = msg;
                    }
                });
        }
        
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