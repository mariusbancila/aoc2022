use crate::utils;
use std::{path::Path};

pub fn execute() {
    println!("=== puzzle 20 ===");

    let test_input = read_numbers("./data/input20test.txt");
    let test_sum = get_mixed_value(&test_input, 1, 1);
    assert_eq!(3, test_sum);

    let input = read_numbers("./data/input20.txt");
    let sum = get_mixed_value(&input, 1, 1);
    println!("{}", sum);

    let test_sum2 = get_mixed_value(&test_input, 10, 811589153);
    assert_eq!(1623178306, test_sum2);

    let sum2 = get_mixed_value(&input, 10, 811589153);
    println!("{}", sum2);

    println!();
}

fn read_numbers<P>(filename : P) -> Vec<i64>
where P : AsRef<Path> {
    if let Ok(lines) = utils::read_lines(filename) {
        let nums : Vec<i64> = lines.into_iter().map(|line| if let Ok(n) = line {return n.parse::<i64>().unwrap(); } else {panic!("Not a line!")}).collect();
        return nums;
    }
    panic!("Invalid file!");
}

// solution from https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/20.rs
fn get_mixed_value(input : &Vec<i64>, count : i32, key : i64) ->i64 {
    let transformed_input : Vec<i64> = input.iter().map(|e| e * key).collect();
    let mut indexes : Vec<usize> = (0..input.len()).collect();

    // rotate the indexes
    for _ in 0..count {
        for (index, &value) in transformed_input.iter().enumerate() {
            let pos = indexes.iter().position(|&i | i == index).unwrap();
            indexes.remove(pos);

            let insertion = (pos as i64 + value).rem_euclid(indexes.len() as i64) as usize;

            indexes.insert(insertion, index);
        }
    }

    // find the index of the value 0
    let input_zero_index = transformed_input.iter().position(|&v| v == 0).unwrap();
    let zero_index = indexes.iter().position(|&i| i == input_zero_index).unwrap();

    // find the elements after specified positions
    let sum = [1000, 2000, 3000].iter().map(|i| transformed_input[indexes[(zero_index + i) % indexes.len()]]).sum();

    sum
}
