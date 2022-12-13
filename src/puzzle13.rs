use crate::utils;
use std::{path::Path, cmp::Ordering};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Number(i32)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(n1), Packet::Number(n2)) => n1.cmp(n2),
            (Packet::List(l1), Packet::List(l2)) => {
                for i in 0..l1.len().min(l2.len()) {
                    let order = l1[i].cmp(&l2[i]);
                    match order {
                        Ordering::Equal => (),
                        _ => return order
                    }
                }

                l1.len().cmp(&l2.len())
            },
            (Packet::Number(n), Packet::List(_)) => Packet::List(vec![Packet::Number(*n)]).cmp(other),
            (Packet::List(_), Packet::Number(n)) => self.cmp(&Packet::List(vec![Packet::Number(*n)]))
        }
    }
}

fn end_of_bracket(chars : &Vec<char>, index : usize) -> usize {
    let mut pairs = 0;
    for i in index..chars.len() {
        if chars[i] == '[' {
            pairs += 1;
        }
        else if chars[i] == ']' {
            if pairs == 0 {
                return i;
            }
            else {
                pairs -= 1;
            }
        }
    }

    chars.len()
}

fn next_comma(chars : &Vec<char>, index : usize) -> usize {
    for i in index..chars.len() {
        if chars[i] == ',' {
            return i
        }
    }

    chars.len()
}

fn from_string(text : &str) -> Packet {
    let mut items : Vec<Packet> = vec![];

    if !(text.starts_with('[') && text.ends_with(']')) {
        panic!("Invalid format!");
    }

    let chars : Vec<char> = text[1..text.len()-1].chars().collect();
    if !chars.is_empty() {
        let mut i : usize = 0;

        while i < chars.len() {
            if chars[i] == '[' {
                let end = end_of_bracket(&chars, i+1);
                let innertext = &text[i+1..end+2];
                let inner = from_string(innertext);

                items.push(inner);

                i = end + 1;
            }
            else if chars[i] == ','{
                i += 1;
            }
            else {
                let end = next_comma(&chars, i);
                let innertext = &text[i+1..end+1];
                let value = innertext.parse::<i32>().unwrap();
                
                items.push(Packet::Number(value));

                i = end + 1;
            }
        }
    }

    Packet::List(items)
}

fn find_ordered_pairs<P>(filename : P) -> usize
where P : AsRef<Path> {
    let mut sum = 0;

    if let Ok(lines) = utils::read_lines(filename) {
        let all_lines : Vec<Result<String, std::io::Error>> = lines.collect();

        let count : usize = (all_lines.len() + 1)/3;

        for i in 0..count {
            if let Ok(line1) = &all_lines[i*3] {
                if let Ok(line2) = &all_lines[i*3+1] {
                    let p1 = from_string(line1);
                    let p2 = from_string(line2);

                    if let Ordering::Less = p1.cmp(&p2) {
                        sum += i + 1;
                    }
                }
            }
        }
    }

    sum
}

fn decode_distress_signal<P>(filename : P) -> usize
where P : AsRef<Path> {
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

    let mut packages : Vec<Packet> = vec![div1.clone(), div2.clone()];

    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(pac) = line {
                if !pac.is_empty() {
                    packages.push(from_string(&pac));
                }
            }
        }
    }

    packages.sort();

    let pos1 = packages.iter().position(|x| *x == div1).unwrap();
    let pos2 = packages.iter().position(|x| *x == div2).unwrap();

    (pos1 + 1) * (pos2 + 1)
}

pub fn execute() {
    println!("=== puzzle 13 ===");
 
    let test_sum = find_ordered_pairs("./data/input13test.txt");
    assert_eq!(13, test_sum);

    let sum = find_ordered_pairs("./data/input13.txt");
    println!("sum={}", sum);

    let test_key = decode_distress_signal("./data/input13test.txt");
    assert_eq!(140, test_key);

    let key = decode_distress_signal("./data/input13.txt");
    println!("key={}", key);

    println!();
}