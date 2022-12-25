use crate::utils::read_lines;
use std::{path::Path};

pub fn execute() {
    println!("=== puzzle 25 ===");

    tests();

    let test_sum = read_numbers("./data/input25test.txt");
    assert_eq!(4890, test_sum);
    let test_snafu = decimal_to_snafu(test_sum);
    assert_eq!("2=-1=0", test_snafu);

    let sum = read_numbers("./data/input25.txt");
    let snafu = decimal_to_snafu(sum);
    println!("sum(snafu): {}", snafu);

    println!();
}

fn tests() {
    assert_eq!(1, snafu_to_decimal("1"));
    assert_eq!(2, snafu_to_decimal("2"));
    assert_eq!(3, snafu_to_decimal("1="));
    assert_eq!(4, snafu_to_decimal("1-"));
    assert_eq!(5, snafu_to_decimal("10"));
    assert_eq!(6, snafu_to_decimal("11"));
    assert_eq!(7, snafu_to_decimal("12"));
    assert_eq!(8, snafu_to_decimal("2="));
    assert_eq!(9, snafu_to_decimal("2-"));
    assert_eq!(10, snafu_to_decimal("20"));
    assert_eq!(15, snafu_to_decimal("1=0"));
    assert_eq!(20, snafu_to_decimal("1-0"));
    assert_eq!(2022, snafu_to_decimal("1=11-2"));
    assert_eq!(12345, snafu_to_decimal("1-0---0"));
    assert_eq!(314159265, snafu_to_decimal("1121-1110-1=0"));    
}

fn snafu_to_decimal(text : &str) -> i64 {
    let mut number = 0i64;

    let mut power = 1;
    for c in text.chars().rev() {
        match c {
            '0'..='9' => {
                number = number + c.to_digit(10).unwrap() as i64 * power;
            },
            '-' => {
                number = number - 1 * power;
            },
            '=' => {
                number = number -2 * power;
            }
            _   => {panic!("Invalid character!")}
        };
        power = power * 5;
    }

    number
}

fn decimal_to_snafu(mut number : i64) -> String {    
    let mut digits : Vec<char> = Vec::new();

    while number > 0{
        let r = number % 5;
        digits.push(match r {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!("Not possible!")
        });

        if r >= 3 {
            number += 5 - r;
        }

        number = number / 5;
    }

    digits.iter().rev().collect::<String>()
}

fn read_numbers<P>(filename: P) -> i64 
where P : AsRef<Path> {
    let mut sum = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                sum = sum + snafu_to_decimal(&number);
            }
        }
    }

    sum
}