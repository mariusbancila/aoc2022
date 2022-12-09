use crate::utils;
use std::collections::HashSet;
use std::path::Path;
use std::fmt::{self};

// Define a struct to represent a point in 2D space
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

type PointSet = HashSet<Point>;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn run_simulation<P>(filename : P) -> PointSet
where P : AsRef<Path> {
    let mut points = PointSet::new();
    let mut current_head = Point{x: 0, y: 0};
    let mut current_tail = Point{x: 0, y: 0};
    points.insert(current_tail);

    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(cmd) = line {
                let parts : Vec<&str> = cmd.split_ascii_whitespace().collect();
                let dir = parts[0];
                let step = parts[1].parse::<u32>().unwrap();

                // println!("[{}|{}]", dir, step);

                for _ in 0..step {                    
                    let (next_head, next_tail) = move_direction(&current_head, &current_tail, dir.chars().next().unwrap());

                    // println!("  H:{},T:{} => H:{},T:{}", current_head, current_tail, next_head, next_tail);

                    if !points.contains(&next_tail) {
                        points.insert(next_tail.clone());
                    }

                    current_head = next_head;
                    current_tail = next_tail;
                }
            }
        }
    }

    points
}

fn move_direction(head : &Point, tail : &Point, dir : char) -> (Point, Point) {
    let mut next_head = *head;
    let mut next_tail = *tail;

    match dir {
        'U' => {
            next_head.y -= 1;            
        },
        'D' => {
            next_head.y += 1;
        },
        'R' => {
            next_head.x += 1;
        },
        'L' => {
            next_head.x -= 1;
        },
        _ => panic!("Unexpected command!")
    }

    if (next_head.x - next_tail.x).abs() > 1 ||
       (next_head.y - next_tail.y).abs() > 1 {
        // same line
        if next_head.y == next_tail.y {
            match dir {
                'R' => {
                    next_tail.x += 1;
                },
                'L' => {
                    next_tail.x -= 1;
                },
                _ => panic!("Unexpected command!")
            }
        }
        // same column
        else if next_head.x == next_tail.x {
            match dir {
                'U' => {
                    next_tail.y -= 1;            
                },
                'D' => {
                    next_tail.y += 1;
                },
                _ => panic!("Unexpected command!")
            }
        }
        // diagonal 
        else {
            // *H    *H
            // ** => *T
            // T*    **
            if (next_head.x == next_tail.x + 1) && (next_head.y == next_tail.y - 2) {
                next_tail.x += 1;
                next_tail.y -= 1;
            }
            // H*    **
            // ** => T*
            // *T    **
            else if (next_head.x == next_tail.x - 1) && (next_head.y == next_tail.y - 2) {
                next_tail.x -= 1;
                next_tail.y -= 1;
            }
            // *T    **
            // ** => T*
            // H*    H*
            else if (next_head.x == next_tail.x - 1) && (next_head.y == next_tail.y + 2) {
                next_tail.x -= 1;
                next_tail.y += 1;
            }
            // T*    **
            // ** => *T
            // *H    *H
            else if (next_head.x == next_tail.x + 1) && (next_head.y == next_tail.y + 2) {
                next_tail.x += 1;
                next_tail.y += 1;
            }
            // **H     *TH
            // T**  => ***
            else if (next_head.x == next_tail.x + 2) && (next_head.y == next_tail.y - 1) {
                next_tail.x += 1;
                next_tail.y -= 1;
            }
            // H**     HT*
            // **T  => ***
            else if (next_head.x == next_tail.x - 2) && (next_head.y == next_tail.y - 1) {
                next_tail.x -= 1;
                next_tail.y -= 1;
            }
            // **T     ***
            // H**  => HT*
            else if (next_head.x == next_tail.x - 2) && (next_head.y == next_tail.y + 1) {
                next_tail.x -= 1;
                next_tail.y += 1;
            }
            // T**     ***
            // **H  => *TH
            else if (next_head.x == next_tail.x + 2) && (next_head.y == next_tail.y + 1) {
                next_tail.x += 1;
                next_tail.y += 1;
            }
        }
    }

    (next_head, next_tail)
}

pub fn execute() {
    println!("=== puzzle 9 ===");

    let test_positions = run_simulation("./data/input09test.txt");
    assert_eq!(13, test_positions.len());

    let positions = run_simulation("./data/input09.txt");
    println!("positions={}", positions.len());

    println!();
}