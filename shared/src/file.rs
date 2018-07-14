use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;


pub fn read_file(path: &Path) -> String {
    let content = read_lines(path)
        .iter()
        .fold(String::new(), |mut content, ref line| {
            content.push_str(line);
            content
        });
    content
}

pub fn read_lines(path: &Path) -> Vec<String> {
    match File::open(path) {
        Err(_) => Vec::new(),
        Ok(file_handler) => {
            let file = BufReader::new(&file_handler);  
            file.lines()
                .filter_map(|line| line.ok())
                .filter(|line| line.is_ascii())
                .collect()
        }
    }
}