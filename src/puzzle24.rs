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
    let test_dist = find_distance(&test_map, test_begin, test_end);
    assert_eq!(18, test_dist);

    let (map, begin, end) = read_map("./data/input24.txt");
    let dist = find_distance(&map, begin, end);
    println!("dist={}", dist);

    println!();
}

fn find_distance(map : &Map, begin : (i32,i32), end: (i32, i32)) -> i32 {
    let mut step = 1i32;
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

                if 0 <= row  && row < height && 0 <= col && col < width &&
                    map.element_at(as_usize(row), as_usize((col - step).rem_euclid(width))).unwrap() != WIND_RIGHT &&
                    map.element_at(as_usize(row), as_usize((col + step).rem_euclid(width))).unwrap() != WIND_LEFT &&
                    map.element_at(as_usize((row - step).rem_euclid(height)), as_usize(col)).unwrap() != WIND_DOWN && 
                    map.element_at(as_usize((row + step).rem_euclid(height)), as_usize(col)).unwrap() != WIND_UP {
                    next_positions.insert((row,col));
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
        shrunk_data.extend_from_slice(&map.data[r * cols + 1..=(r + 1)* cols - 1]);
    }

    (Map::new_from(rows - 2, cols - 2, shrunk_data), (start_row-1, start_col-1),(end_row-1, end_col-1))
}

/*
const EMPTY : u8        = 0x00;
const WALL : u8         = 0xFF;
const WIND_UP : u8      = 0x01;
const WIND_DOWN : u8    = 0x02;
const WIND_LEFT : u8    = 0x04;
const WIND_RIGHT : u8   = 0x08;

type Map = algebra::Matrix<u8>;

pub fn execute() {
    println!("=== puzzle 24 ===");

    let test_map = read_map("./data/input24test.txt");
    let test_dist = find_distance(&test_map);
    assert_eq!(18, test_dist);

    println!();
}

fn find_distance(map : &Map) -> u32 {
    let states = build_states(map);

    let ((start_row, start_col),(end_row, end_col)) = find_entry_exit(map);

    let mut crt_row = start_row;
    let mut crt_col = start_col;

    let mut steps = 0;

    while crt_row != end_row && crt_col != end_col {
        steps += 1;
    }

    steps
}

fn find_entry_exit(map: &Map) -> ((usize, usize),(usize, usize)) {
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

    ((0, entry_col),(map.rows-1, exit_col))
}

fn build_states(map: &Map) -> Vec<Map> {
    let mut maps : Vec<Map> = Vec::new();

    maps.push(map.clone());

    let size = num::integer::lcm(map.rows, map.cols);    

    for i in 0..size {
        let mut data : Vec<u8> = Vec::new();

        // copy top row (walls)
        data.copy_from_slice(&maps[i].data[0..maps[i].cols]);

        for r in 1..maps[i].rows-1 {
            // copy left (wall)
            data.push(maps[i].element_at(r, 0).unwrap());

            // build middle
            for c in 1..maps[i].cols-1 {
                let left_col = if c - 1 > 0 {c-1} else {maps[i].cols - 2};
                let right_col = if c + 1 < maps[i].cols - 1 {c + 1} else {1};
                let top_row = if r - 1 > 0 {r - 1} else {maps[i].rows - 2};
                let bottom_row = if r + 1 < maps[i].rows - 1 {r + 1} else {1};

                data.push(
                    // down wind from upper cell
                    (maps[i].element_at(top_row, c).unwrap() & WIND_DOWN) |
                    // up wind from lower cell
                    (maps[i].element_at(bottom_row, c).unwrap() & WIND_UP) |
                    // right wind from left cell
                    (maps[i].element_at(r, left_col).unwrap() & WIND_RIGHT) | 
                    // left wind from right cell
                    (maps[i].element_at(r, right_col).unwrap() & WIND_LEFT)
                );
            }

            // copy right (wall)
            data.push(maps[i].element_at(r, maps[i].cols-1).unwrap());
        }

        // copy last row (walls)
        data.copy_from_slice(&maps[i].data[(maps[i].rows - 1) * maps[i].cols..maps[i].rows * maps[i].cols]);

        maps.push(Map::new_from(maps[i].rows, maps[i].cols, data));
    }

    maps
}

fn read_map<P>(filename : P) -> Map 
where P : AsRef<Path> {
    let mut data : Vec<u8> = Vec::new();
    let mut rows = 0;
    let mut cols = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(text) = &line {
                for c in text.chars() {
                    data.push(match c {
                        '#' => WALL,
                        '.' => EMPTY,
                        '^' => WIND_UP,
                        'v' => WIND_DOWN,
                        '<' => WIND_LEFT,
                        '>' => WIND_RIGHT,
                        _  => panic!("Unrecognized input!")
                    });
                }

                if cols == 0 {
                    cols = text.len();
                }
                rows += 1;
            }
        }
    }

    Map::new_from(rows, cols, data)
}
*/