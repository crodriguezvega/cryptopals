use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_file(path: &Path) -> Result<(), &'static str> {
    if !path.exists() {
        Err("File not found")
    } else {
        let file_handler = File::open(path).unwrap();
        let file = BufReader::new(&file_handler);
        for line in file.lines() {
            let l = line.unwrap();
            println!("{}", l); 
        }

        Ok(())
    }
}