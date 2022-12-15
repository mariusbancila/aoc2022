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

struct L1BallRegion {
    top : Point2D,
    bottom : Point2D,
    left : Point2D,
    right : Point2D
}

#[derive(Copy, Clone)]
struct Line {
    from : Point2D,
    to   : Point2D
}

impl Line {
    fn new(f : &Point2D, t : &Point2D) -> Line {
        Line {from : *f, to : *t}
    }
}

fn make_l1ball_region(sensor: &Point2D, beacon : &Point2D) -> L1BallRegion {
    let mdist = manchester_distance(&sensor, &beacon) + 1;

    L1BallRegion {
        top : Point2D::new(sensor.x, sensor.y - mdist),
        bottom : Point2D::new(sensor.x, sensor.y + mdist),
        left : Point2D::new(sensor.x - mdist, sensor.y),
        right : Point2D::new(sensor.x + mdist, sensor.y),
    }
}

fn counter_clock_wise(a : &Point2D, b: &Point2D, c: &Point2D) -> bool {
    (c.y-a.y)*(b.x-a.x) > (b.y-a.y)*(c.x-a.x)
}

fn are_intersecting(l1 : &Line, l2 : &Line) -> bool {
    counter_clock_wise(&l1.from, &l2.from, &l2.to) != counter_clock_wise(&l1.to, &l2.from, &l2.to) &&
    counter_clock_wise(&l1.from, &l1.to, &l2.from) != counter_clock_wise(&l1.from, &l1.to, &l2.to)
}

fn intersection(l1 : &Line, l2 : &Line) -> Option<Point2D> {
    let a1 = l1.to.y - l1.from.y;
    let b1 = l1.from.x - l1.to.x;
    let c1 = a1 * l1.from.x + b1 * l1.from.y;

    let a2 = l2.to.y - l2.from.y;
    let b2 = l2.from.x - l2.to.x;
    let c2 = a2 * l2.from.x + b2 * l2.from.y;

    let d = a1*b2 - a2*b1;

    if d != 0 {
        let x = (b2*c1 - b1*c2)/d;
        let y = (a1*c2 - a2*c1)/d;

        return Some(Point2D::new(x, y));
    }

    None
}

fn find_tunning_frequency(snb : &SensorsAndBeacons, maxline : i32) -> i64 {
    let mut regions : Vec<L1BallRegion> = Vec::new();

    // STEP 1: determine the l1-ball regions of exclusions for each sensor
    
    // Example : l1-ball exlusion region for a sensor-beacon with Manhatan distance 4
    //           # - position where a beacon cannot be located
    //           . - empty space
    //           @ - position where a beacon can be located (the outer margin of the l1-ball of each sensor)

    // .....@.....      .....@.....
    // ....@#@....      ....@.@....
    // ...@###@...      ...@...@...
    // ..@####B@..      ..@....B@..
    // .@#######@.      .@.......@.
    // @####S####@  =>  @....S....@
    // .@#######@.      .@.......@.
    // ..@#####@..      ..@.....@..
    // ...@###@...      ...@...@...
    // ....@#@....      ....@.@....
    // .....@.....      .....@.....

    for (sensor, beacon) in snb {
        regions.push(make_l1ball_region(sensor, beacon));
    }

    // STEP 2: determine the intersection of each pair of l1-ball regions
    //         build a list of line segments defining the intersection polygon(s)

    // Example : intersections of two l1-ball regions
    //           @ - point on margin of the region
    //           * - point on the margin that is included in another exlusion region and therefore of no intereset

    // .....@.........
    // ....@.@...@....
    // ...@...@.@.@...
    // ..@....B@...@..
    // .@.....*.*..B@.
    // @....S*...S...@
    // .@.....*.*...@.
    // ..@.....@...@..
    // ...@...@.@.@...
    // ....@.@...@....
    // .....@.........

    let mut segments : Vec<Line> = Vec::new();

    for i in 0..regions.len()-1 {
        let l1 = &regions[i];
        let l2 = &regions[i+1];
    }

    // STEP 3: check the interction polygon against the search area

    0
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

    //let test_freq = find_tunning_frequency(&test_snb, 20);
    //assert_eq!(56000011, test_freq);

    println!();
}