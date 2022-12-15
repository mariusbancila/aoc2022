use crate::utils::{self};
use crate::algebra::{Point2D, SparseMatrix};
use std::{path::Path};

const ROCK : char = '#';
const SAND : char = 'o';

type Matrix = SparseMatrix<char>;

fn parse_matrix<P>(filename : P) -> Matrix
where P : AsRef<Path> {
    let mut matrix = Matrix::new();

    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(path) = line {
                let paths : Vec<&str> = path.split(" -> ").collect();
                let mut points : Vec<Point2D> = Vec::new();

                for p in paths {
                    let parts : Vec<&str> = p.split(',').collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse::<i32>().unwrap();
                        let y = parts[1].parse::<i32>().unwrap();    

                        points.push(Point2D::new(x, y));
                    }
                }

                for i in 0..points.len() - 1 {
                    let start = &points[i];
                    let end = &points[i+1];

                    // vertical line
                    if start.x == end.x {
                        if start.y <= end.y {
                            for y in start.y..=end.y {
                                matrix.insert(start.x, y, ROCK);
                            }
                        }
                        else {
                            for y in end.y..=start.y {
                                matrix.insert(start.x, y, ROCK);
                            }
                        }
                    }
                    // horizontal line
                    else if start.y == end.y { 
                        if start.x <= end.x {
                            for x in start.x..=end.x {
                                matrix.insert(x, start.y, ROCK);
                            }
                        }
                        else {
                            for x in end.x..=start.x {
                                matrix.insert(x, start.y, ROCK);
                            }
                        }
                    }
                    
                    if start.y > matrix.bottom_most {
                        matrix.bottom_most = start.y;
                    }
                    if end.y > matrix.bottom_most {
                        matrix.bottom_most = end.y;
                    }
                    if start.x < matrix.left_most {
                        matrix.left_most = start.x;
                    }
                    if start.x > matrix.right_most {
                        matrix.right_most = start.x;
                    }
                    if end.x < matrix.left_most {
                        matrix.left_most = end.x;
                    }
                    if end.x > matrix.right_most {
                        matrix.right_most = end.x;
                    }
                }                
            }
        }
    }

    matrix
}

fn find_units_of_sand(matrix : &mut Matrix) -> i32 {
    let mut count = 0;

    loop {
        let mut x = 500;
        let mut y = 0;

        while y < matrix.bottom_most {
            if let None = matrix.element_at(x, y+1) {
                y += 1;
                continue;
            }
            if let None = matrix.element_at(x - 1, y + 1) {
                y += 1;
                x -= 1;
                continue;
            }
            if let None = matrix.element_at(x + 1, y + 1) {
                y += 1;
                x += 1;
                continue;
            }            

            break;
        }

        if y == matrix.bottom_most {
            break;
        }

        if !matrix.try_insert(x, y, SAND) {
            panic!("Cannot insert unit!");
        }
        count += 1;
    }    

    count
}

fn find_units_of_sand2(matrix : &mut Matrix) -> i32 {
    let mut count = 0;

    loop {
        let mut x = 500;
        let mut y = 0;

        while y < matrix.bottom_most+1 {
            if let None = matrix.element_at(x, y+1) {
                y += 1;
                continue;
            }
            if let None = matrix.element_at(x - 1, y + 1) {
                y += 1;
                x -= 1;
                continue;
            }
            if let None = matrix.element_at(x + 1, y + 1) {
                y += 1;
                x += 1;
                continue;
            }

            break;
        }

        if y == 0 {
            break;
        }

        if !matrix.try_insert(x, y, SAND) {
            panic!("Cannot insert unit!");
        }
        count += 1;
    }    

    count + 1
}

pub fn execute() {
    println!("=== puzzle 14 ===");

    let test_matrix = parse_matrix("./data/input14test.txt");
    assert_eq!(24, find_units_of_sand(&mut test_matrix.clone()));

    let matrix = parse_matrix("./data/input14.txt");
    let count = find_units_of_sand(&mut matrix.clone());
    println!("units={}", count);

    assert_eq!(93, find_units_of_sand2(&mut test_matrix.clone()));

    let count2 = find_units_of_sand2(&mut matrix.clone());
    println!("units={}", count2);
    
    println!();
}