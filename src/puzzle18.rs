use crate::utils;
use std::{path::Path, collections::HashSet};

#[derive(Clone,Copy,Eq,PartialEq,PartialOrd,Hash)]
struct Point3d(i32,i32,i32);

type Space = HashSet<Point3d>;

fn parse_space<P>(filename : P) -> Space
where P : AsRef<Path> {
    let mut space : Space = HashSet::new();

    if let Ok(lines) = utils::read_lines(filename) {       
        for line in lines {
            if let Ok(point) = line {
                let parts : Vec<&str> = point.split(',').collect();
                if parts.len() == 3 {
                    let p = Point3d(
                        parts[0].parse::<i32>().unwrap(),
                        parts[1].parse::<i32>().unwrap(),
                        parts[2].parse::<i32>().unwrap());

                    space.insert(p);
                }
                else {
                    panic!("Unexpected format!");
                }
            }
        }
    }    
    space
}

fn count_exposed_sides(space : &Space) -> u32 {
    let mut sides = 0;

    let dx : [i32; 6] = [-1, 1, 0,  0, 0,  0];
    let dy : [i32; 6] = [ 0, 0, 1, -1, 0,  0];
    let dz : [i32; 6] = [ 0, 0, 0,  0, 1, -1];

    for point in space {
        for i in 0..6 {
            let adjacent = Point3d(point.0 + dx[i], point.1 + dy[i], point.2 + dz[i]);

            if !space.contains(&adjacent){
                sides += 1;
            }
        }
    }

    sides
}

pub fn execute() {
    println!("=== puzzle 18 ===");

    let test_space = parse_space("./data/input18test.txt");
    let test_count = count_exposed_sides(&test_space);
    assert_eq!(64, test_count);

    let space = parse_space("./data/input18.txt");
    let count = count_exposed_sides(&space);
    println!("sides={}", count);

    println!();
}