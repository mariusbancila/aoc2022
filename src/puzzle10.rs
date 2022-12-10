use crate::utils;
use std::path::Path;

fn sum_of_signal_strenghts<P>(filename : P) -> i32
where P : AsRef<Path> {
    let mut sum = 0;

    if let Ok(lines) = utils::read_lines(filename) {
        let mut cycle = 1;
        let mut regx = 1;

        for line in lines {
            if let Ok(cmd) = line {
                let parts : Vec<&str> = cmd.split_ascii_whitespace().collect();
                let op = parts[0];
                let value = if parts.len() == 2 { parts[1].parse::<i32>().unwrap() } else {0};

                match op {
                    "noop" => {
                        cycle += 1;
                    },
                    "addx" => {
                        cycle += 1;

                        if is_verifiable_cycle(&cycle) {
                            sum += cycle * regx;
                        }

                        cycle += 1;

                        regx += value;
                    },
                    _ => panic!("Unknown operation!")
                }

                if is_verifiable_cycle(&cycle) {
                    sum += cycle * regx;
                }
            }
        }
    }    

    sum
}

fn is_verifiable_cycle(cycle : &i32) -> bool {
    *cycle == 20 || (*cycle <= 220 && (*cycle - 20) % 40 == 0)
}

fn draw_screen<P>(filename : P)
where P : AsRef<Path> {
    if let Ok(lines) = utils::read_lines(filename) {
        let mut cycle = 0;
        let mut regx = 1;

        for line in lines {
            if let Ok(cmd) = line {
                let parts : Vec<&str> = cmd.split_ascii_whitespace().collect();
                let op = parts[0];
                let value = if parts.len() == 2 { parts[1].parse::<i32>().unwrap() } else {0};

                match op {
                    "noop" => {
                        draw_pixel(&cycle, &regx);

                        cycle += 1;
                    },
                    "addx" => {
                        draw_pixel(&cycle, &regx);

                        cycle += 1;

                        check_and_reset_cycle(&mut cycle);

                        draw_pixel(&cycle, &regx);

                        cycle += 1;

                        regx += value;
                    },
                    _ => panic!("Unknown operation!")
                }      
                
                check_and_reset_cycle(&mut cycle); 
            }
        }
    }    
    println!();

}

fn is_sprite_in_sync(cycle : &i32, regx : &i32) -> bool {
    *cycle >= *regx-1 && *cycle <= *regx + 1
}

fn draw_pixel(cycle : &i32, regx : &i32) {
    if is_sprite_in_sync(&cycle, &regx) {
        print!("#");
    }
    else {
        print!(".");
    }

    if (*cycle+1) % 40 == 0 {
        println!();
    }
}

fn check_and_reset_cycle(cycle : &mut i32) {
    if *cycle % 40 == 0 {
        *cycle = 0;
    } 
}

pub fn execute() {
    println!("=== puzzle 10 ===");

    let test_sum = sum_of_signal_strenghts("./data/input10test.txt");
    assert_eq!(13140, test_sum);

    let sum = sum_of_signal_strenghts("./data/input10.txt");
    println!("sum={}", sum);

    draw_screen("./data/input10test.txt");

    println!();

    draw_screen("./data/input10.txt");
    println!();

    println!();
}