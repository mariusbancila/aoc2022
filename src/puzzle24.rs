use crate::{utils::{read_lines, as_i32, as_usize}, algebra::{self}};
use std::{path::Path, collections::HashSet};

const EMPTY : char        = '.';
const WIND_UP : char      = '^';
const WIND_DOWN : char    = 'v';
const WIND_LEFT : char    = '<';
const WIND_RIGHT : char   = '>';

type Map = algebra::Matrix<char>;

pub fn execute() {
    println!("=== puzzle 24 ===");

    let (test_map, test_begin, test_end) = read_map("./data/input24test.txt");
    let mut test_dist = find_distance(&test_map, test_begin, test_end, 1);
    assert_eq!(18, test_dist);

    test_dist = find_distance(&test_map, test_end, test_begin, test_dist);
    test_dist = find_distance(&test_map, test_begin, test_end, test_dist);
    assert_eq!(54, test_dist);


    let (map, begin, end) = read_map("./data/input24.txt");
    let mut dist = find_distance(&map, begin, end, 1);
    println!("dist={}", dist);

    dist = find_distance(&map, end, begin, dist);
    dist = find_distance(&map, begin, end, dist);

    println!("dist 3x={}", dist);

    println!();
}

fn find_distance(map : &Map, begin : (i32,i32), end: (i32, i32), mut step : i32) -> i32 {
    let (start_row, start_col) = begin;
    let (end_row, end_col) = end;

    let mut positions : HashSet<(i32, i32)> = HashSet::new();

    let height = as_i32(map.rows);
    let width = as_i32(map.cols);

    'outer : loop {
        let mut next_positions : HashSet<(i32, i32)> = HashSet::new();

        for position in &positions {

            let possible_moves = vec![
                (position.0,    position.1),    // same cell
                (position.0 - 1,position.1),    // up cell
                (position.0 + 1,position.1),    // down cell
                (position.0,    position.1 - 1),// left cell
                (position.0,    position.1 + 1) // right cell
            ];

            for (row, col) in possible_moves {
                if (row,col) == (end_row, end_col) {
                    break 'outer;
                }

                if 0 <= row && row < height && 0 <= col && col < width {
                    let left_cell = map.element_at(as_usize(row), as_usize((col - step).rem_euclid(width))).unwrap();
                    let right_cell = map.element_at(as_usize(row), as_usize((col + step).rem_euclid(width))).unwrap();
                    let top_cell = map.element_at(as_usize((row - step).rem_euclid(height)), as_usize(col)).unwrap();
                    let bottom_cell = map.element_at(as_usize((row + step).rem_euclid(height)), as_usize(col)).unwrap();

                    if left_cell != WIND_RIGHT && right_cell != WIND_LEFT && top_cell != WIND_DOWN && bottom_cell != WIND_UP {
                        next_positions.insert((row,col));
                    }
                }
            }
        }

        positions = next_positions;

        if positions.is_empty() {
            positions.insert((start_row, start_col));
        }

        step += 1;
    }

    step
}

fn find_entry_exit(map: &Map) -> ((i32, i32),(i32, i32)) {
    let mut entry_col = 0;
    let mut exit_col = 0;

    for c in 0..map.cols {
        if map.element_at(0, c).unwrap() == EMPTY {
            entry_col = c;
            break;
        }
    }

    for c in 0..map.cols {
        if map.element_at(map.rows - 1, c).unwrap() == EMPTY {
            exit_col = c;
            break;
        }
    }

    (
        (0, as_i32(entry_col)), 
        (as_i32(map.rows-1), as_i32(exit_col))
    )
}

fn read_map<P>(filename : P) -> (Map, (i32, i32),(i32, i32)) 
where P : AsRef<Path> {
    let mut data : Vec<char> = Vec::new();
    let mut rows = 0;
    let mut cols = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(text) = &line {
                let mut chs = text.chars().collect::<Vec<char>>();
                data.append(&mut chs);

                if cols == 0 {
                    cols = text.len();
                }
                rows += 1;
            }
        }
    }    

    let map = Map::new_from(rows, cols, data);

    let ((start_row, start_col),(end_row, end_col)) = find_entry_exit(&map);

    let mut shrunk_data : Vec<char> = Vec::new();
    for r in 1..rows-1 {
        shrunk_data.extend_from_slice(&map.data[r * cols+1..=(r + 1)* cols - 2]);
    }

    (Map::new_from(rows - 2, cols - 2, shrunk_data), (start_row-1, start_col-1),(end_row-1, end_col-1))
}