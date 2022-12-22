use crate::algebra;
use crate::utils::read_lines;
use std::{path::Path};

const EMPTY : char = ' ';
const OPEN : char = '.';
const WALL : char = '#';

type Grid = algebra::Matrix<char>;

#[derive(Copy,Clone,PartialEq, Eq)]
enum Direction {
    Right(usize),
    Left(usize)
}

#[derive(Copy,Clone,PartialEq, Eq)]
enum Facing {
    Right,
    Down,
    Left,
    Up
}

type MoveInstructions = Vec<Direction>;

type Bounds = Vec<(usize,usize)>;

pub fn execute() {
    println!("=== puzzle 22 ===");

    let (test_grid, test_moves) = read_grid("./data/input22test.txt", 16);
    let test_password = follow_instructions(&test_grid, &test_moves);
    assert_eq!(6032, test_password);

    let (grid, moves) = read_grid("./data/input22.txt", 150);
    let password = follow_instructions(&grid, &moves);
    println!("password={}", password);

    println!();
}

fn read_grid<P>(filename: P, max_len : usize) -> (Grid, MoveInstructions)
where P : AsRef<Path> {
    let mut cols = max_len;
    let mut rows = 0;
    let mut data : Vec<char> = Vec::new();
    let mut instructions = MoveInstructions::new();

    if let Ok(lines) = read_lines(filename) {
        let all_lines = lines.collect::<Vec<_>>();

        loop {
            if let Ok(line) = &all_lines[rows] {
                if line.is_empty() {
                    break;
                }                
                let mut chars = line.chars().collect::<Vec<char>>();
                chars.resize(max_len, EMPTY);

                data.append(&mut chars);

                rows += 1;
            }
        }

        if let Ok(last_line) = &all_lines[rows+1] {
            let first_move = "R".to_owned();
            let complete = first_move + &last_line;
            let mut moves = parse_directions(&complete);
            instructions.append(&mut moves);
        }        
    }

    (Grid::new_from(rows, cols, data), instructions)
}

fn parse_directions(text : &str) -> MoveInstructions {
    let mut instructions = Vec::new();

    let mut start = 0usize;
    let mut end = 0usize;

    let chars = text.chars().collect::<Vec<char>>();

    while end < text.len() {
        let dir = chars[start];
        end = start + 1;

        while end < text.len() {
            if chars[end].is_numeric() {
                end += 1;
            }
            else {
                break;
            }
        }

        let steps = text[start+1..end].parse::<usize>().unwrap();

        match dir {
            'R' => {
                instructions.push(Direction::Right(steps));
            },
            'L' => {
                instructions.push(Direction::Left(steps));
            },
            _ => panic!("Unexpected direction")
        }

        start = end;
    }

    instructions
}

fn find_start(grid : &Grid) -> (usize, usize) {
    for c in 0..grid.cols {
        if let Some(e) = grid.element_at(0, c) {
            if e == OPEN {
                return (0, c)
            }
        }
    }

    panic!("Start position not found on first row!");
}

fn reface(current : Facing, dir : Direction) -> Facing {
    match current {
        Facing::Up => {
            match dir {
                Direction::Left(_) => Facing::Left,
                Direction::Right(_) => Facing::Right
            }
        },
        Facing::Left => {
            match dir {
                Direction::Left(_) => Facing::Down,
                Direction::Right(_) => Facing::Up
            }
        },
        Facing::Down => {
            match dir {
                Direction::Left(_) => Facing::Right,
                Direction::Right(_) => Facing::Left
            }
        }
        Facing::Right => {
            match dir {
                Direction::Left(_) => Facing::Up,
                Direction::Right(_) => Facing::Down
            }
        }
    }
}

fn facing_value(facing : Facing) -> usize {
    match facing {
        Facing::Up => 3,
        Facing::Left => 2,
        Facing::Down => 1,
        Facing::Right => 0
    }   
}

fn find_row_bounds(grid : &Grid) -> Bounds {
    let mut row_bounds = Bounds::new();

    for r in 0..grid.rows {
        let mut lower = 0;
        let mut upper = 0;
        for c in 0..grid.cols {
            if grid.element_at(r, c).unwrap() != EMPTY {
                lower = c;
                break;
            }
        }

        for c in (lower+1..grid.cols).rev() {
            if grid.element_at(r, c).unwrap() != EMPTY {
                upper = c;
                break;
            }
        }

        row_bounds.push((lower, upper));
    }

    row_bounds
}

fn find_col_bounds(grid : &Grid) -> Bounds {
    let mut col_bounds = Bounds::new();

    for c in 0..grid.cols {
        let mut lower = 0;
        let mut upper = 0;
        for r in 0..grid.rows {
            if grid.element_at(r, c).unwrap() != EMPTY {
                lower = r;
                break;
            }
        }

        for r in (lower+1..grid.rows).rev() {
            if grid.element_at(r, c).unwrap() != EMPTY {
                upper = r;
                break;
            }
        }

        col_bounds.push((lower, upper));
    }

    col_bounds
}

fn follow_instructions(grid : &Grid, instructions : &MoveInstructions) -> usize {
    let (mut crt_row, mut crt_col) = find_start(&grid);

    // keep track of the limits of each rows and columns
    // rows keep column bounds, cols keep row bounds
    let row_bounds = find_row_bounds(&grid);
    let col_bounds = find_col_bounds(&grid);

    let mut facing = Facing::Up;

    for instruction in instructions {
        facing = reface(facing, *instruction);

        let steps = match instruction {
            Direction::Left(left) => left,
            Direction::Right(right) => right
        };

        for _ in 0..*steps {
            let (next_row, next_col) = next_position(crt_row, crt_col, facing, &row_bounds, &col_bounds);

            if let Some(e) = grid.element_at(next_row, next_col) {
                match e {
                    OPEN => {
                        crt_row = next_row;
                        crt_col = next_col;
                    },
                    WALL => {
                        break;
                    },
                    EMPTY => {
                        panic!("Empty path not possible!")
                    },
                    _ => panic!("Unexpected path!")
                }
            }
        }
    }

    1000 * (crt_row + 1)+ 4 * (crt_col + 1) + facing_value(facing)
}

fn next_position(row: usize, col : usize, facing : Facing, row_bounds : &Bounds, col_bounds : &Bounds) -> (usize, usize) {
    match facing {
        Facing::Up => {
            if row > col_bounds[col].0 {
                return (row - 1, col);
            }
            else {
                return (col_bounds[col].1, col)
            }
        },
        Facing::Left => {
            if col > row_bounds[row].0 {
                return (row, col - 1)
            }
            else {
                return (row, row_bounds[row].1)
            }
        },
        Facing::Down => {
            if row < col_bounds[col].1 {
                return (row + 1, col);
            }
            else {
                return (col_bounds[col].0, col)
            }
        },
        Facing::Right => {
            if col < row_bounds[row].1 {
                return (row, col + 1)
            }
            else {
                return (row, row_bounds[row].0)
            }
        }
    }
}
