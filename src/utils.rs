use std::io::{self, BufRead};
use std::fs::{self, File};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file_string<P>(filename: P) -> Result<String, Box<dyn std::error::Error>> 
where P: AsRef<Path> {
    let data = fs::read_to_string(filename)?;
    Ok(data)
}