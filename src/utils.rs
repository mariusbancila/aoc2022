use std::io::{self, BufRead};
use std::fs::{self, File};
use std::path::Path;

#[allow(unused)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(unused)]
pub fn read_file_string<P>(filename: P) -> Result<String, Box<dyn std::error::Error>> 
where P: AsRef<Path> {
    let data = fs::read_to_string(filename)?;
    Ok(data)
}

#[allow(unused)]
pub fn as_i32(n : usize) -> i32 {
    i32::try_from(n).unwrap()
}

#[allow(unused)]
pub fn as_usize(n : i32) -> usize {
    usize::try_from(n).unwrap()
}