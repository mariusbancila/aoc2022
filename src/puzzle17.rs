use crate::utils;
use std::path::Path;

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
            for byte in rock.bytes() {
                if height < cave.len() {
                    cave[height] |= byte;
                }
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

fn eval_rock_tower_hight(jets : &JetVector, rocks : &RockCollection, count : usize) -> usize {
    let mut cave : Cave = Vec::with_capacity(count * 4);
    let mut jet_index = 0;

    for block in rocks.into_iter().cycle().take(count) {
        jet_index = simulate_rock(&mut cave, jets, jet_index, *block).unwrap();
    }

    cave.len()
}

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
    let test_height = eval_rock_tower_hight(&test_jets, &rocks, 2022);    
    assert_eq!(3068, test_height);

    let jets = get_gas_jets("./data/input17.txt");
    let height = eval_rock_tower_hight(&jets, &rocks, 2022);
    println!("height={}", height);

    println!();
}