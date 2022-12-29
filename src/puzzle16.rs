use regex::Regex;
use crate::utils::{self, read_lines};
use std::{path::Path, collections::HashMap};

pub fn execute() {
    println!("=== puzzle 16 ===");

    let test_valves = parse_data("./data/input16test.txt");

    let valves = parse_data("./data/input16.txt");

    println!();
}

struct Valve {
    name : String,
    flow_rate : u32,
    connections : Vec<String>
}

impl Valve {
    fn new(n : &str, rate : u32, valves : Vec<String>) -> Valve {
        Valve { name: n.to_string(), flow_rate: rate, connections: valves }
    }
}

type ValveCollection = HashMap<String, Valve>;

fn parse_data<P>(filename : P) -> ValveCollection 
where P : AsRef<Path> {
    let mut valves : ValveCollection = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        let re: Regex = Regex::new("Valve (\\w+) has flow rate=(\\d+); tunnels? leads? to valves? (.+)").unwrap();

        for line in lines {
            if let Ok(text) = line {
                let caps = re.captures(&text).unwrap();

                let valve = caps.get(1).unwrap().as_str().to_string();
                let flow = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let connections = caps.get(3).unwrap().as_str();

                let parts = connections.split(", ").map(|t|t.to_string()).collect::<Vec<String>>();
                
                valves.insert(valve.clone(), Valve::new(&valve, flow, parts));
            }
        }
    }

    valves
}