use crate::algebra::{self, SparseMatrix};
use crate::utils::read_lines;
use std::{path::Path, collections::HashMap};

const ELF : char = '#';

const DX : [i32; 8] = [-1,  0,  1, 1, 1, 0, -1, -1];
const DY : [i32; 8] = [-1, -1, -1, 0, 1, 1,  1,  0];

//                           North      South      West       East
const DXARR : [[i32;3];4] = [[-1, 0, 1],[-1, 0, 1],[-1,-1,-1],[ 1, 1, 1]];
const DYARR : [[i32;3];4] = [[-1,-1,-1],[ 1, 1, 1],[-1, 0, 1],[-1, 0, 1]];

const DXMOV : [i32; 4] = [ 0, 0,-1, 1];
const DYMOV : [i32; 4] = [-1, 1, 0, 0];

type Elf = u32;

type Position = algebra::Point2D;
type Proposals = HashMap<Position, u16>;
type Grid = SparseMatrix<Elf>;

pub fn execute() {
    println!("=== puzzle 23 ===");

    let test_grid = parse_grid("./data/input23test.txt");
    let test_tiles = simulate(&test_grid, 10);
    assert_eq!(110, test_tiles);

    let grid = parse_grid("./data/input23.txt");
    let tiles = simulate(&grid, 10);
    println!("tiles={}", tiles);

    let test_rounds = find_stable_stage(&test_grid);
    assert_eq!(20, test_rounds);

    let rounds = find_stable_stage(&grid);
    println!("rounds={}", rounds);

    println!();
}

fn parse_grid<P>(filename : P) -> Grid 
where P : AsRef<Path> {
    let mut grid = Grid::new();
    let mut elves = 0;

    if let Ok(lines) = read_lines(filename) {
        let mut y = 0;

        for line in lines {
            if let Ok(text) = line {
                let chars = text.chars().collect::<Vec<char>>();
                for x in 0..chars.len() {
                    if chars[x] == ELF {
                        elves += 1;

                        grid.insert(i32::try_from(x).unwrap(), y, elves);
                    }
                }
            }

            y += 1;
        }
    }

    grid
}

fn has_neighbors(grid: &Grid, position : Position) -> bool {
    for i in 0..8 {
        if let Some(_) = grid.element_at(position.x + DX[i], position.y + DY[i]) {
            return true;
        }
    }
    false
}

fn find_next_position(grid: &Grid, position : Position, round : usize) -> Option<Position> {
    let mut direction : usize = round % 4;

    for _ in 0..4 {
        let mut found = false;
        for i in 0..3 {
            let x = position.x + DXARR[direction][i];
            let y = position.y + DYARR[direction][i];

            if let Some(_) = grid.element_at(x, y) {
                found = true;
                break;
            }            
        }

        if !found {
            let newx = position.x + DXMOV[direction];
            let newy = position.y + DYMOV[direction];
            return Some(Position::new(newx, newy));
        }

        direction = (direction + 1) % 4;
    }

    None
}

fn simulate(grid: &Grid, rounds : usize) -> u32 {
    let mut crt_grid = grid.clone();

    for round in 0..rounds {
        // part 1: make proposals
        let mut proposals_count = Proposals::new();
        let mut elf_proposals : HashMap<u32, Position> = HashMap::new();

        for elf in &crt_grid.points {
            if has_neighbors(&crt_grid, *elf.0) {
                if let Some(proposed_pos) = find_next_position(&crt_grid, *elf.0, round) {
                    // can make a proposal

                    if let Some(v) = proposals_count.get_mut(&proposed_pos) {
                        *v += 1;
                    }
                    else {
                        proposals_count.insert(proposed_pos, 1);
                    }

                    // map elf id to proposed position
                    elf_proposals.insert(*elf.1, proposed_pos);
                }
            }
        }

        // part 2: move
        let mut next_grid : Grid = Grid::new();

        // check each elf
        for elf in &crt_grid.points {
            // elf.0 = position
            // elf.1 = id

            let mut moved = false;
            // if the elf made a proposal
            if let Some(elf_prop) = elf_proposals.get(&elf.1) {
                // count the proposals for that point
                if let Some(v) = proposals_count.get(&elf_prop) {
                    // if there is only one proposal then move the elf
                    if *v == 1 {
                        next_grid.insert(elf_prop.x, elf_prop.y, *elf.1);
                        moved = true;
                    }
                }
            }

            if !moved {
                next_grid.insert(elf.0.x, elf.0.y, *elf.1);
            }
        }

        // swap grids
        crt_grid = next_grid;
    }

    let mut lowerx = i32::MAX;
    let mut upperx = i32::MIN;
    let mut lowery = i32::MAX;
    let mut uppery = i32::MIN;

    for elf in &crt_grid.points {
        if lowerx > elf.0.x {
            lowerx = elf.0.x;
        }
        if upperx < elf.0.x {
            upperx = elf.0.x;
        }

        if lowery > elf.0.y {
            lowery = elf.0.y;
        }
        if uppery < elf.0.y {
            uppery = elf.0.y;
        }        
    }

    let mut count = 0;

    for y in lowery..=uppery {
        for x in lowerx..=upperx {
            if let Some(_) = crt_grid.element_at(x, y) {
                count += 1;
            }
        }    
    }

    u32::try_from(upperx - lowerx + 1).unwrap() * u32::try_from(uppery - lowery + 1).unwrap() - count
}

fn find_stable_stage(grid: &Grid) -> usize {
    let mut crt_grid = grid.clone();
    let mut round = 0;

    loop {
        // part 1: make proposals
        let mut proposals_count = Proposals::new();
        let mut elf_proposals : HashMap<u32, Position> = HashMap::new();

        for elf in &crt_grid.points {
            if has_neighbors(&crt_grid, *elf.0) {
                if let Some(proposed_pos) = find_next_position(&crt_grid, *elf.0, round) {
                    // can make a proposal

                    if let Some(v) = proposals_count.get_mut(&proposed_pos) {
                        *v += 1;
                    }
                    else {
                        proposals_count.insert(proposed_pos, 1);
                    }

                    // map elf id to proposed position
                    elf_proposals.insert(*elf.1, proposed_pos);
                }
            }
        }

        // part 2: move
        let mut next_grid : Grid = Grid::new();

        let mut changed = false;
        // check each elf
        for elf in &crt_grid.points {
            // elf.0 = position
            // elf.1 = id

            let mut moved = false;
            // if the elf made a proposal
            if let Some(elf_prop) = elf_proposals.get(&elf.1) {
                // count the proposals for that point
                if let Some(v) = proposals_count.get(&elf_prop) {
                    // if there is only one proposal then move the elf
                    if *v == 1 {
                        next_grid.insert(elf_prop.x, elf_prop.y, *elf.1);
                        moved = true;
                        changed = true;
                    }
                }
            }

            if !moved {
                next_grid.insert(elf.0.x, elf.0.y, *elf.1);
            }
        }

        // swap grids
        crt_grid = next_grid;

        round += 1;

        if !changed {
            break;
        }
    }

    round
}