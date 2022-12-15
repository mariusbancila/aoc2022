use regex::Regex;
use crate::{utils::{self}, algebra::{SparseMatrix, Point2D}};
use std::{path::Path, collections::HashMap};

const SENSOR : char = 'S';
const BEACON : char = 'B';
const IMPOSIBLE_POSITION : char = '#';

type Matrix = SparseMatrix<char>;
type SensorsAndBeacons = HashMap<Point2D, Point2D>;

fn parse_matrix<P>(filename : P) -> (Matrix, SensorsAndBeacons)
where P : AsRef<Path> {
    let mut matrix = Matrix::new();
    let mut snb = SensorsAndBeacons::new();

    if let Ok(lines) = utils::read_lines(filename) {
        let re: Regex = Regex::new("Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)").unwrap();

        for line in lines {
            if let Ok(text) = line {
                let caps = re.captures(&text).unwrap();
   
                let sx = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let sy = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let bx = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let by = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

                snb.insert(Point2D::new(sx, sy), Point2D::new(bx, by));

                if !matrix.try_insert(sx, sy, SENSOR) {
                    panic!("Invalid sensor position!");
                }

                if !matrix.try_insert(bx, by, BEACON) {
                    panic!("Invalid sensor position!");
                }

                if sy > matrix.bottom_most {
                    matrix.bottom_most = sy;
                }
                if sy < matrix.top_most {
                    matrix.top_most = sy;
                }

                if by > matrix.bottom_most {
                    matrix.bottom_most = by;
                }
                if by < matrix.top_most {
                    matrix.top_most = by;
                }

                if sx < matrix.left_most {
                    matrix.left_most = sx;
                }
                if sx > matrix.right_most {
                    matrix.right_most = sx;
                }
                if bx < matrix.left_most {
                    matrix.left_most = bx;
                }
                if bx > matrix.right_most {
                    matrix.right_most = bx;
                }                
            }
        }
    }

    (matrix, snb)
}

fn manchester_distance(p1 : &Point2D, p2 : &Point2D) -> i32 {
    i32::abs(p1.x - p2.x) + i32::abs(p1.y - p2.y)
}

fn find_impossible_positions(matrix : &mut Matrix, snb : &SensorsAndBeacons, line : i32) -> usize {
    for (sensor, beacon) in &*snb {
        let mdist = manchester_distance(&sensor, &beacon);

        for dy in 0..=mdist {
            let y = mdist - dy;
            for dx in -dy..=dy {                
                if let None = matrix.element_at(sensor.x + dx, sensor.y - y) {
                    matrix.try_insert(sensor.x + dx, sensor.y - y, IMPOSIBLE_POSITION);
                }            
            }
        }
        for dy in 1..=mdist {
            for dx in -(mdist-dy)..=(mdist-dy) {
                if let None = matrix.element_at(sensor.x + dx, sensor.y + dy) {
                    matrix.try_insert(sensor.x + dx, sensor.y + dy, IMPOSIBLE_POSITION);
                }
            }
        }
    }

    matrix.points.iter().filter(|&(k,v)|k.y == line && *v == IMPOSIBLE_POSITION).count()
}

fn find_impossible_positions_smart(matrix : &mut Matrix, snb : &SensorsAndBeacons, line : i32) -> usize {
    for (sensor, beacon) in &*snb {
        let mdist = manchester_distance(&sensor, &beacon);

        if sensor.y - mdist <= line && line <= sensor.y + mdist {
            if line <= sensor.y {
                let y = sensor.y - line;
                let dy = mdist - y;
                for dx in -dy..=dy {                
                    if let None = matrix.element_at(sensor.x + dx, sensor.y - y) {
                        matrix.try_insert(sensor.x + dx, sensor.y - y, IMPOSIBLE_POSITION);
                    }            
                }
            }
            else {
                let dy = line - sensor.y;
                for dx in -(mdist-dy)..=(mdist-dy) {
                    if let None = matrix.element_at(sensor.x + dx, sensor.y + dy) {
                        matrix.try_insert(sensor.x + dx, sensor.y + dy, IMPOSIBLE_POSITION);
                    }
                }
            }
        }
    }

    matrix.points.iter().filter(|&(k,v)|k.y == line && *v == IMPOSIBLE_POSITION).count()
}

pub fn execute() {
    println!("=== puzzle 15 ===");

    let (test_matrix, test_snb) = parse_matrix("./data/input15test.txt");
    let test_count = find_impossible_positions(&mut test_matrix.clone(), &test_snb, 10);
    assert_eq!(26, test_count);

    let test_count2 = find_impossible_positions_smart(&mut test_matrix.clone(), &test_snb, 10);
    assert_eq!(26, test_count2);

    let (matrix, snb) = parse_matrix("./data/input15.txt");
    let count = find_impossible_positions_smart(&mut matrix.clone(), &snb, 2000000);
    println!("count={}", count);

    println!();
}