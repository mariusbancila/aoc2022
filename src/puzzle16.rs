use priority_queue::PriorityQueue;
use regex::Regex;
use crate::utils::{read_lines};
use std::{path::Path, collections::{HashMap, HashSet}, hash::Hash, cmp::{Ordering}};

// https://github.com/vss2sn/advent_of_code/blob/master/2022/cpp/day_16a.cpp

pub fn execute() {
    println!("=== puzzle 16 ===");

    let test_valves = parse_data("./data/input16test.txt");
    let test_dist = find_distances(&test_valves);
    let test_max_flow = find_max_flow(&test_valves, &test_dist);
    assert_eq!(1651, test_max_flow);

    let valves = parse_data("./data/input16.txt");

    println!();
}

struct Valve {
    name : String,
    flow_rate : i32,
    connections : Vec<String>
}

impl Valve {
    fn new(n : &str, rate : i32, valves : Vec<String>) -> Valve {
        Valve { name: n.to_string(), flow_rate: rate, connections: valves }
    }
}

type ValveCollection = HashMap<String, Valve>;

#[derive(Debug, Eq, Clone, Hash)]
struct ReverseNumber {
    value : i32
}

impl PartialOrd for ReverseNumber {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ReverseNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value < other.value {
            Ordering::Greater
        }
        else if self.value == other.value {
            Ordering::Equal
        }
        else {
            Ordering::Less
        }
    }
}

impl PartialEq for ReverseNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn parse_data<P>(filename : P) -> ValveCollection 
where P : AsRef<Path> {
    let mut valves : ValveCollection = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        let re: Regex = Regex::new("Valve (\\w+) has flow rate=(\\d+); tunnels? leads? to valves? (.+)").unwrap();

        for line in lines {
            if let Ok(text) = line {
                let caps = re.captures(&text).unwrap();

                let valve = caps.get(1).unwrap().as_str().to_string();
                let flow = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let connections = caps.get(3).unwrap().as_str();

                let parts = connections.split(", ").map(|t|t.to_string()).collect::<Vec<String>>();
                
                valves.insert(valve.clone(), Valve::new(&valve, flow, parts));
            }
        }
    }

    valves
}

fn find_distances(valves : &ValveCollection) -> HashMap<String, HashMap<String, i32>> {
    let mut distances : HashMap<String, HashMap<String, i32>> = HashMap::new();

    for (name,_) in valves {
        let d = find_distances_for(name, valves);

        distances.insert(name.clone(), d);
    }

    distances
}

fn find_distances_for(valve : &String, valves : &ValveCollection) -> HashMap<String, i32> {
    let mut distances : HashMap<String, i32> = HashMap::new();
    let mut visited : HashSet<String> = HashSet::new();
    let mut queue : PriorityQueue<String, ReverseNumber> = PriorityQueue::new();

    queue.push(valve.clone(), ReverseNumber { value: 0 });

    while !queue.is_empty() {
        let current = queue.pop().unwrap();

        if visited.contains(&current.0) {
            continue;
        }

        visited.insert(current.0.clone());

        distances.insert((*current.0).to_string(), current.1.value);

        let connections = &valves.get_key_value(&current.0).unwrap().1.connections;

        for connection in connections {
            queue.push(connection.clone(), ReverseNumber { value: current.1.value + 1});
        }
    }

    distances
}

fn get_all_paths(valves : &ValveCollection, 
                 source : &str, 
                 time : i32,
                 distances : &HashMap<String, HashMap<String, i32>>) -> Vec<Vec<String>> {
    let mut paths : Vec<Vec<String>> = Vec::new();
    let mut path : Vec<String> = vec![source.to_string()];
    let mut visited : HashSet<String> = HashSet::new();

    visited.insert(source.to_string());

    depth_first_search(&mut path, &mut paths, &mut visited, valves, source.to_string(), time, distances);

    paths
}

fn depth_first_search(path : &mut Vec<String>, 
                      paths : &mut Vec<Vec<String>>,
                      visited : &mut HashSet<String>,
                      valves : &ValveCollection,
                      current : String,
                      time : i32,
                      distances : &HashMap<String, HashMap<String, i32>>) {
    if time == 0 {
        paths.push(path.clone());
        return;
    }

    for (valve,dist) in distances.get_key_value(&current).unwrap().1 {
        if visited.contains(valve) {
            continue;
        }

        if time - dist - 1 < 0 {
            paths.push(path.clone());
            continue;
        }

        if valves.get_key_value(valve).unwrap().1.flow_rate == 0 {
            continue;
        }

        visited.insert(valve.clone());
        path.push(valve.clone());

        depth_first_search(path, paths, visited, valves, valve.clone(), time - dist - 1, distances);
    
        visited.remove(valve);
        path.pop();
    }

    paths.push(path.clone());
}

fn calc_flow(path : &Vec<String>, 
             time : &mut i32, 
             valves : &ValveCollection, 
             distances: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut score : i32 = 0;
    for i in 0..path.len() - 1 {
        let map = distances.get_key_value(&path[i]).unwrap().1;
        let dist = map.get_key_value(&path[i+1]).unwrap().1;
        *time = *time - dist - 1;

        score += valves.get_key_value(&path[i+1]).unwrap().1.flow_rate * (*time);
    }

    score
}

fn find_max_flow(valves : &ValveCollection, distances: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut max_flow = 0;
    let mut time = 30;

    let paths = get_all_paths(valves, "AA", 30, distances);
    for path in paths {
        let flow = calc_flow(&path, &mut time, valves, distances);

        if flow > max_flow {
            max_flow = flow;
        }
    }

    max_flow
}