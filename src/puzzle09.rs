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

fn run_simulation<P>(filename : P, length: usize) -> PointSet
where P : AsRef<Path> {
    let mut points = PointSet::new();
    let mut rope : Vec<Point> = Vec::new();
    for _ in 0..length {
        rope.push(Point{x: 0, y: 0})
    }
    points.insert(Point{x: 0, y: 0});

    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(cmd) = line {
                let parts : Vec<&str> = cmd.split_ascii_whitespace().collect();
                let dir = parts[0].chars().next().unwrap();
                let step = parts[1].parse::<usize>().unwrap();

                for _ in 1..=step {           
                    rope[0] = move_head(&rope[0], dir);

                    for i in 1..length {
                        let next_tail = move_tail(&rope[i-1], &rope[i], dir);

                        rope[i] = next_tail;
                    }

                    if !points.contains(&rope[length-1]) {
                        points.insert(rope[length-1].clone());
                    }
                }
            }
        }
    }

    points
}

fn move_head(head : &Point, dir : char) -> Point {
    let mut next_head = *head;

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
        _ => panic!("Unexpected command '{}'!", dir)
    }

    next_head
}

fn move_tail(next_head : &Point, tail : &Point, dir : char) -> Point {
    let mut next_tail = *tail;

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
                'U'|'D' => {
                    if next_head.x < next_tail.x {
                        next_tail.x -= 1;
                    }
                    else if next_head.x > next_tail.x {
                        next_tail.x += 1;
                    }
                }                
                _ => panic!("Unexpected command '{}'!", dir)
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
                'L'|'R' => {
                    if next_head.y < next_tail.y {
                        next_tail.y -= 1;
                    }
                    else if next_head.y > next_tail.y {
                        next_tail.y += 1;
                    }
                }
                _ => panic!("Unexpected command '{}'!", dir)
            }
        }
        // diagonal 
        else {
            // *HH    *HH
            // **H => *TH
            // T**    ***
            if (next_head.x >= next_tail.x + 1) && (next_head.y <= next_tail.y -1 ) {
                next_tail.x += 1;
                next_tail.y -= 1;
            }
            // HH*    HH*
            // H** => HT*
            // **T    ***
            else if (next_head.x <= next_tail.x - 1) && (next_head.y <= next_tail.y - 1) {
                next_tail.x -= 1;
                next_tail.y -= 1;
            }
            // **T    ***
            // H** => HT*
            // HH*    HH*
            else if (next_head.x <= next_tail.x - 1) && (next_head.y >= next_tail.y + 1) {
                next_tail.x -= 1;
                next_tail.y += 1;
            }
            // T**    ***
            // **H => *TH
            // *HH    *HH
            else if (next_head.x >= next_tail.x + 1) && (next_head.y >= next_tail.y + 1) {
                next_tail.x += 1;
                next_tail.y += 1;
            }
        }
    }

    next_tail
}

pub fn execute() {
    println!("=== puzzle 9 ===");

    let test_positions = run_simulation("./data/input09test1.txt", 2);
    assert_eq!(13, test_positions.len());

    let positions = run_simulation("./data/input09.txt", 2);
    println!("positions={}", positions.len());

    let test_positions2 = run_simulation("./data/input09test1.txt", 10);
    assert_eq!(1, test_positions2.len());

    let test_positions3 = run_simulation("./data/input09test2.txt", 10);
    assert_eq!(36, test_positions3.len());

    let positions2 = run_simulation("./data/input09.txt", 10);
    println!("positions={}", positions2.len());

    println!();
}