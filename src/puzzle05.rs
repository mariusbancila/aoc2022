use crate::utils;
use std::path::Path;
use std::collections::VecDeque;
use regex::Regex;

enum ParsingStage {
    Stacks,
    Moves
}

pub fn execute() {
    println!("=== puzzle 5 ===");

    let top_test = get_top_stacks("./data/input05test.txt", true);
    assert_eq!("CMZ", top_test.unwrap());

    let top_result = get_top_stacks("./data/input05.txt", true);
    println!("top={}", top_result.unwrap());

    let top_test2 = get_top_stacks("./data/input05test.txt", false);
    assert_eq!("MCD", top_test2.unwrap());

    let top_result2 = get_top_stacks("./data/input05.txt", false);
    println!("top={}", top_result2.unwrap());

    println!();
}

pub fn get_top_stacks<P>(filename : P, as_stack : bool) -> Result<String, String>
where P: AsRef<Path> {
    if let Ok(lines) = utils::read_lines(filename) {
        let re: Regex = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
        let mut stack_count: Option<usize> = None;

        let mut stage =  ParsingStage::Stacks;
        let mut stacks: Vec<VecDeque<char>> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                let chs: Vec<char> = ip.chars().collect();
                let len = chs.len();

                match stage {
                    ParsingStage::Stacks => {
                        match stack_count {
                            Some(_) => {},
                            None => {
                                let count = (len + 1)/4 as usize;
                                stack_count = Some(count);

                                for _ in 1..=count {
                                    stacks.push(VecDeque::new());
                                }
                            }
                        };

                        if chs.iter().any(| &x| x == '[') {
                            for i in 0..stack_count.unwrap() {
                                let l = chs[i*4];
                                let m = chs[i*4+1];
                                let r = chs[i*4+2];

                                if l == '[' && r == ']' {
                                    stacks[i].push_back(m);
                                }                                
                            }
                        }
                        else {
                            stage = ParsingStage::Moves;
                        }
                    },
                    ParsingStage::Moves => {
                        if ip.chars().count() == 0 {
                            continue;
                        }

                        if let Some(caps) = re.captures(&ip) {   
                            let n = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                            let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

                            if as_stack {
                                for _ in 1..=n {
                                    if let Some(v) = stacks[from-1].pop_front() {
                                        stacks[to-1].push_front(v);
                                    }
                                }
                            }
                            else {
                                let mut temp : VecDeque<char> = VecDeque::new();
                                for _ in 1..=n {
                                    if let Some(v) = stacks[from-1].pop_front() {
                                        temp.push_front(v);
                                    }
                                }
    
                                for e in temp {
                                    stacks[to-1].push_front(e);
                                }                                
                            }
                        }
                    }
                }
            }
        }

        let mut result = String::new();
        for i in 0..stack_count.unwrap() {
            if let Some(v) = stacks[i].front() {
                result.push(*v);
            }
        }

        return Ok(result);
    }

    Err(String::from("Cannot read lines"))
}