use crate::utils;
use std::{path::Path, collections::VecDeque};
use std::fmt::{self};
trait Computation {
    fn evaluate(&self, old: u64) -> u64;
}

struct OldTimesOld;
struct OldPlusOld;
struct OldTimesValue {
    value : u64
}
struct OldPlusValue {
    value : u64
}

impl Computation for OldTimesOld {
    fn evaluate(&self, old: u64) -> u64 {
        old * old
    }
}

impl Computation for OldPlusOld {
    fn evaluate(&self, old: u64) -> u64 {
        old + old
    }
}

impl Computation for OldTimesValue {
    fn evaluate(&self, old: u64) -> u64 {
        old * self.value
    }
}

impl Computation for OldPlusValue {
    fn evaluate(&self, old: u64) -> u64 {
        old + self.value
    }
}

struct Monkey {
    items : VecDeque<u64>,
    computation : Box<dyn Computation>,
    divisor : u64,
    next_monkey_if_true : usize,
    next_monkey_if_false : usize
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = writeln!(f, "monkey");
        if let Err(_) = result {
            return result;
        }

        for e in &self.items {
            result = write!(f, "{}, ", e);
            if let Err(_) = result {
                return result;
            }
        }
        result = writeln!(f);
        if let Err(_) = result {
            return result;
        }

        result = write!(f, "div={}, next_true={}, next_false={}", self.divisor, self.next_monkey_if_true, self.next_monkey_if_false);
        if let Err(_) = result {
            return result;
        }

        writeln!(f)
    }
}

impl Monkey {
    fn new(levels : VecDeque<u64>, c : Box<dyn Computation>, div : u64, ntrue : usize, nfalse : usize) -> Monkey {
        Monkey { items: levels, computation: c, divisor: div, next_monkey_if_true: ntrue, next_monkey_if_false: nfalse }
    }

    fn push(&mut self, item : u64) {
        self.items.push_back(item);
    }

    fn clear(&mut self) {
        self.items.clear();
    }
}

fn test_computations() {
    let computations : Vec<Box<dyn Computation>> = vec![
        Box::new(OldTimesOld{}),
        Box::new(OldPlusOld{}),
        Box::new(OldTimesValue{value : 10}),
        Box::new(OldPlusValue{value : 10})
    ];

    let expected : Vec<u64> = vec![9, 6, 30, 13];

    for i in 0..computations.len() {
        assert_eq!(expected[i], computations[i].evaluate(3));
    }
}

fn parse_monkeys<P>(filename : P) -> Vec<Monkey> 
where P : AsRef<Path> {
    let mut monkeys : Vec<Monkey> = Vec::new();

    if let Ok(lines) = utils::read_lines(filename) {
        let all_lines : Vec<Result<String, std::io::Error>> = lines.collect();

        let count : usize = (all_lines.len() + 1)/7;
        for i in 0..count {
            //Monkey 0:
            if let Ok(line1) = &all_lines[i*7] {
                let index = line1[7..line1.len()-1].parse::<usize>().unwrap();
                assert_eq!(i, index);
            }

            //  Starting items: 79, 98
            let mut items : VecDeque<u64> = VecDeque::new();
            if let Ok(line2) = &all_lines[i*7+1] {
                let parts : VecDeque<&str> = line2[18..line2.len()].split(", ").collect();
                for p in parts {
                    items.push_back(p.parse::<u64>().unwrap());
                }
            }

            //  Operation: new = old * old
            //  Operation: new = old + old
            //  Operation: new = old * 19
            //  Operation: new = old + 5
            let mut computation : Option<Box<dyn Computation>> = None;
            if let Ok(line3) = &all_lines[i*7+2] {
                let operation = line3[13..line3.len()].to_string();
                if operation == "new = old * old" {
                    computation = Some(Box::new(OldTimesOld));
                }
                else if operation == "new = old + old" {
                    computation = Some(Box::new(OldPlusOld));
                }
                else if operation.starts_with("new = old * ") {
                    let v = line3[25..line3.len()].parse::<u64>().unwrap();
                    computation = Some(Box::new(OldTimesValue{value: v}));
                }
                else if operation.starts_with("new = old + ") {
                    let v = line3[25..line3.len()].parse::<u64>().unwrap();
                    computation = Some(Box::new(OldPlusValue{value: v}));
                }
                else {
                    panic!("Unknown operation type: {}!", operation);
                }
            }

            //  Test: divisible by 23
            let mut divisor : u64 = 0;
            if let Ok(line4) = &all_lines[i*7+3] {
                divisor = line4[21..line4.len()].parse::<u64>().unwrap();
            }

            //    If true: throw to monkey 2
            let mut true_monkey : usize = 0;
            if let Ok(line5) = &all_lines[i*7+4] {
                true_monkey = line5[29..line5.len()].parse::<usize>().unwrap();
            }

            //    If false: throw to monkey 3
            let mut false_monkey : usize = 0;
            if let Ok(line6) = &all_lines[i*7+5] {
                false_monkey = line6[30..line6.len()].parse::<usize>().unwrap();
            }

            monkeys.push(Monkey::new(items, computation.unwrap(), divisor, true_monkey, false_monkey))
        }
    }

    monkeys
}

fn compute_monkey_business(monkeys: &mut Vec<Monkey>, rounds : i32, divide : bool, val : u64) -> u64 {
    let mut inspections : Vec<u64> = Vec::new();
    inspections.resize(monkeys.len(), 0);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for level_index in 0..monkeys[i].items.len() {
                inspections[i] += 1;

                let old_level = monkeys[i].items[level_index];
                let eval_level = monkeys[i].computation.evaluate(old_level);                
                let new_level = if divide {eval_level / val} else {eval_level % val};
                if new_level % monkeys[i].divisor == 0 {
                    let index = monkeys[i].next_monkey_if_true;
                    monkeys[index].push(new_level);
                }
                else {
                    let index = monkeys[i].next_monkey_if_false;
                    monkeys[index].push(new_level);
                }
            }

            monkeys[i].clear();
        }
    }

    inspections.sort_by(|a,b|b.cmp(a));

    inspections[0] * inspections[1]
}

fn compute_common_divisor(monkeys: &Vec<Monkey>) -> u64 {
    let mut divisor = 1;

    for m in monkeys {
        divisor *= m.divisor;
    }

    divisor
}

pub fn execute() {
    println!("=== puzzle 11 ===");

    test_computations();

    let mut test_monkeys = parse_monkeys("./data/input11test.txt");
    assert_eq!(10605, compute_monkey_business(&mut test_monkeys, 20, true, 3));

    let mut monkeys = parse_monkeys("./data/input11.txt");
    let mb = compute_monkey_business(&mut monkeys, 20, true, 3);
    println!("monkey_business={}", mb);

    let mut test_monkeys2 = parse_monkeys("./data/input11test.txt");
    let test_lcm = compute_common_divisor(&test_monkeys2);
    assert_eq!(2713310158, compute_monkey_business(&mut test_monkeys2, 10000, false, test_lcm));

    let mut monkeys2 = parse_monkeys("./data/input11.txt");
    let lcm = compute_common_divisor(&monkeys2);
    let mb = compute_monkey_business(&mut monkeys2, 10000, false, lcm);
    println!("monkey_business2={}", mb);

    println!();
}