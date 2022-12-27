use crate::utils;
use std::{path::Path, collections::{HashMap, hash_map::Entry}};

#[derive(Clone, Copy)]
enum JetDirection {
    Left,
    Right
}

type JetVector = Vec<JetDirection>;

fn get_gas_jets<P>(filename: P) -> JetVector
where P : AsRef<Path> {
    let text = utils::read_file_string(filename);
    
    let jets : JetVector = text.unwrap()
                               .chars()
                               .map(|c| if c == '<' {JetDirection::Left} else if c == '>' {JetDirection::Right} else {panic!("Invalid input!")}).collect();
    jets
}

type Cave = Vec<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rock(u32);

impl Rock {
    const fn intersects(&self, mask : u32) -> bool {
        self.0 & mask != 0
    }

    fn shift(&mut self, dir : JetDirection, mask : u32) {
        let pos = match dir {
            JetDirection::Left => {
                // wall is at 0x80; if left cells are at 0x40 then it cannot shift left
                if self.0 & 0x40404040 == 0 {
                    self.0 << 1
                } 
                else {
                    return;
                }
            },
            JetDirection::Right => {
                // wall is at 0x00; if the right cells are at 0x01 then it cannot shift right
                if self.0 & 0x01010101 == 0 {
                    self.0 >> 1
                }
                else {
                    return;
                }
            }
        }; 

        // if the new position does not intersect fallen rocks then move to the new position
        if pos & mask == 0 {
            self.0 = pos
        }
    }

    fn bytes(self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|b| *b != 0)
    }
}

type RockCollection = [Rock;5];

fn cave_structure(cave: &[u8], height: usize) -> u32 {
    if height >= cave.len() {
        0
    } else {
        cave[height..]
            .iter()
            .take(4)
            .rev()
            .fold(0u32, |acc, b| (acc << 8) | *b as u32)
    }
}

fn simulate_rock(cave : &mut Cave, jets : &JetVector, mut jet_index : usize, mut rock : Rock) -> Option<usize> {
    let mut height = cave.len() + 3;

    loop {
        let shift_dir = jets[jet_index];
        jet_index += 1;

        // loop the jets if necessary
        if jet_index == jets.len() {
            jet_index = 0;
        }

        // get the cave structure
        let mask = cave_structure(cave, height);

        // shift the rock
        rock.shift(shift_dir, mask);

        // the rock cannot move anymore
        if height == 0 || rock.intersects(cave_structure(cave, height - 1)) {
            // check each of the lines of the rock
            for byte in rock.bytes() {
                // if the height is less than the cave height it means the rock has fallen down and stopped somewhere in the cave
                if height < cave.len() {
                    cave[height] |= byte;
                }
                // otherwise the rock is on top of the cave and the cave must grow
                else {
                    cave.push(byte);
                }

                height += 1;
            }

            return Some(jet_index);
        }
        // the rock drop one level
        else {
            height -= 1;
        }
    }
}

fn eval_rock_tower_height(jets : &JetVector, rocks : &RockCollection, total_rocks : usize) -> usize {
    let mut cave : Cave = Vec::with_capacity(total_rocks * 4);
    let mut jet_index = 0;

    for block in rocks.into_iter().cycle().take(total_rocks) {
        jet_index = simulate_rock(&mut cave, jets, jet_index, *block).unwrap();
    }

    cave.len()
}

fn eval_rock_tower_height_long_run(jets : &JetVector, rocks : &RockCollection, total_rocks : u64) -> usize {
    let mut states = HashMap::with_capacity(1024);
    let mut cave : Cave = Vec::with_capacity(1024);

    let rocks_len = u64::try_from(rocks.len()).unwrap();
    let mut cave_height = 0;
    let mut jet_index = 0;
    let mut rocks_count = 0;
    while rocks_count < total_rocks {
        // find the next rock
        let rock_index = usize::try_from(rocks_count % rocks_len).unwrap();

        // simulate rock drop
        jet_index = simulate_rock(&mut cave, jets, jet_index, rocks[rock_index]).unwrap();
        rocks_count += 1;

        if cave.len() < 8 {
            continue;
        }

        // check the current state, made from the top 8 rows, the rock index, and the jet index
        let top = u64::from_ne_bytes(cave[cave.len() - 8..].try_into().unwrap());
        let state = (top, rock_index, jet_index);

        // check the cached states 
        match states.entry(state) {
            // the state was encountered before
            Entry::Occupied(e) => {
                let (prev_rocks_count, prev_height) = e.get();
                // count the rocks in the cycle
                let num_rocks_in_cycle = rocks_count - prev_rocks_count;
                // determine how many cycles are left
                let num_cycles = (total_rocks - rocks_count) / num_rocks_in_cycle;
                // jump the rock count
                rocks_count += num_rocks_in_cycle * num_cycles;
                // increase cave height based on the cave length and number of cycles
                cave_height += num_cycles * u64::try_from(cave.len() - prev_height).unwrap();

                // erase the cache
                states.clear();
            },
            // the state was not found before
            Entry::Vacant(e) => {
                e.insert((rocks_count, cave.len()));
            }
        }
    }

    cave.len() + usize::try_from(cave_height).unwrap()
}

/*
    Solution by https://www.reddit.com/user/Gix/
    https://topaz.github.io/paste/#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
*/
pub fn execute() {
    println!("=== puzzle 17 ===");

    let rocks : RockCollection = [
        Rock(0x0000001E), 
        Rock(0x00081C08),
        Rock(0x0004041C),
        Rock(0x10101010),
        Rock(0x00001818)
    ];

    let test_jets = get_gas_jets("./data/input17test.txt");
    let test_height = eval_rock_tower_height(&test_jets, &rocks, 2022);    
    assert_eq!(3068, test_height);

    let jets = get_gas_jets("./data/input17.txt");
    let height = eval_rock_tower_height(&jets, &rocks, 2022);
    println!("height 1={}", height);

    let test_height_2 = eval_rock_tower_height_long_run(&test_jets, &rocks, 1000000000000);
    assert_eq!(1514285714288, test_height_2);

    let height_2 = eval_rock_tower_height_long_run(&jets, &rocks, 1000000000000);
    println!("height 2={}", height_2);

    println!();
}