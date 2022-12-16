use regex::Regex;
use crate::{utils::{self}, algebra::{SparseMatrix, Point2D}};
use std::{path::Path, collections::HashMap, cmp::Ordering};

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

fn manhattan_distance(p1 : &Point2D, p2 : &Point2D) -> i32 {
    i32::abs(p1.x - p2.x) + i32::abs(p1.y - p2.y)
}

fn find_impossible_positions(matrix : &mut Matrix, snb : &SensorsAndBeacons, line : i32) -> usize {
    for (sensor, beacon) in &*snb {
        let mdist = manhattan_distance(&sensor, &beacon);

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
        let mdist = manhattan_distance(&sensor, &beacon);

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

struct Sensor {
    position : Point2D,
    range : i32
}

fn sign(value : i32) -> i32 {
    if value < 0 {-1} else {1}
}

fn find_tunning_frequency(snb : &SensorsAndBeacons) -> i64 {
    /*
    Look at each pair of sensors and check if their covering range intersects.
    If there is exactly one line between them (their Manhattan's distance is equal to sum of their coverage area + 2)
    then check that line to see if any other scanner is covering it.
    If there is a point along that line not covered by any other sensor, that's the missing beacon!

    ............
    ....1.......
    ...111......
    ..11111.....
    .111S111....
    ..11111@2...
    ...111@222..
    ....1@22S22.
    .......222..
    ........2...
    ............

    1 - the coverage area of sensor 1.
    2 - the coverage area of sensor 2
    @ - point on a line between two neighbors

    mdist(sensor1, beacon1) = 3
    mdist(sensor2, beacon2) = 2
    mdist(sensor1, sensor2) = 7

    Condition:
    mdist(sensor1, beacon1) + mdist(sensor2, beacon2) + 2 = mdist(sensor1, sensor2)
     */

    let mut sensors : Vec<Sensor> = Vec::new();

    for (sensor, beacon) in &*snb {
        sensors.push(Sensor{position : *sensor, range : manhattan_distance(sensor, beacon)});
    }

    let mut dx = 0;
    let mut dy = 0;

    'outer: 
    for i in 0..sensors.len() {
        for j in i+1..sensors.len() {
            let mdist = manhattan_distance(&sensors[i].position, &sensors[j].position);
            if mdist == sensors[i].range + sensors[j].range + 2 {
                let x1 = sensors[i].position.x + sign(sensors[j].position.x - sensors[i].position.x) * (sensors[i].range+1);
                let y1 = sensors[i].position.y;

                let x2 = sensors[i].position.x;
                let y2 = sensors[i].position.y + sign(sensors[j].position.y - sensors[i].position.y) * (sensors[i].range+1);

                // check every point between the coverage ranges
                dx = x1;
                dy = y1;
                while dx != x2 && dy != y2 {
                    // check if the point is out of scan of any other scanner

                    let mut good = true;
                    for k in 0..sensors.len() {
                        if manhattan_distance(&Point2D::new(dx, dy), &sensors[k].position) <= sensors[k].range {
                            good = false;
                            break;
                        }
                    }

                    if good {
                        break 'outer;
                    }

                    dx += sign(x2 - x1);
                    dy += sign(y2 - y1);
                }
            }
        }
    }

    dx as i64 * 4000000 + dy as i64
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

    let test_freq = find_tunning_frequency(&test_snb);
    assert_eq!(56000011, test_freq);

    let freq = find_tunning_frequency(&snb);
    println!("freq={}", freq);

    println!();
}