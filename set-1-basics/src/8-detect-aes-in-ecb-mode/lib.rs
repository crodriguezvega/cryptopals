use std::path::Path;
use itertools::Itertools;
use common::{ file, mappings };

pub fn detect(path: &Path) -> Result<String, &'static str> {
    if !path.exists() {
        Err("File not found")
    }
    else {
        let lines = file::read_lines(path);

        let mut max_score = 0;
        let mut line_number = 0;        
        lines.iter()
            .map(|line| mappings::hex_string_to_bytes(line))
            .enumerate()     
            .for_each(|(i, bytes)| {
                let score = score(&bytes);
                if max_score < score {
                    line_number = i;
                    max_score = score;
                }
            });

        Ok(lines[line_number].clone())
    }
}

fn score(bytes: &[u8]) -> u32 {
    bytes.chunks(16)
        .combinations(2)
        .fold(0, |acc, combination| {
            if combination[0] == combination[1] {
                acc + 1
            } else {
                acc
            }
        })
}