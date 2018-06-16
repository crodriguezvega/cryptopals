use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_lines(path: &Path) -> Vec<String> {
    let file_handler = File::open(path).unwrap();
    let file = BufReader::new(&file_handler);
    
    file.lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.is_ascii())
        .collect()
}