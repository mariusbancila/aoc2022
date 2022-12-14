use std::io::{self, BufRead};
use std::fs::{self, File};
use std::path::Path;
use std::{cmp::Ordering, cmp::Eq};

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

#[derive(Debug, Eq, Clone, Hash)]
pub struct Point2D {
    pub x : i32,
    pub y : i32
}

impl Point2D {
    pub fn new(x : i32, y : i32) -> Point2D {
        Point2D { x, y }
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        }
        else if self.x == other.x {
            if self.y < other.y {
                Ordering::Less
            }
            else if self.y == other.y {
                Ordering::Equal
            }
            else {
                Ordering::Greater
            }
        }
        else {
            Ordering::Greater
        }
    }
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}