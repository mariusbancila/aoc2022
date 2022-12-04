use crate::utils;
use std::path::Path;

use regex::Regex;

pub fn execute() {
   println!("=== puzzle 4 ===");
   
   let count_test = get_completely_overlapping_ranges("./data/input04test.txt");
   assert_eq!(2, count_test);
   
   let count = get_completely_overlapping_ranges("./data/input04.txt");
   println!("count={}", count);
   
   let count_test2 = get_overlapping_ranges("./data/input04test.txt");
   assert_eq!(4, count_test2);
   
   let count2 = get_overlapping_ranges("./data/input04.txt");
   println!("count={}", count2);
   
   println!();
}

fn get_completely_overlapping_ranges<P>(filename : P) -> i32 
where P: AsRef<Path>, {
   let mut count = 0;
   
   if let Ok(lines) = utils::read_lines(filename) {
      
      let re: Regex = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();
      
      for line in lines {
         if let Ok(ip) = line {
            
            let caps = re.captures(&ip).unwrap();
   
            let b1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let e1 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let b2 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let e2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

            if (b1 <= b2 && e1 >= e2) || (b2 <= b1 && e2 >= e1) {
               count += 1;
            }
         }
      }
   }
   
   count
}

fn get_overlapping_ranges<P>(filename : P) -> i32 
where P: AsRef<Path>, {
   let mut count = 0;
   
   if let Ok(lines) = utils::read_lines(filename) {
      
      let re: Regex = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();
      
      for line in lines {
         if let Ok(ip) = line {
            
            let caps = re.captures(&ip).unwrap();
   
            let b1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let e1 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let b2 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let e2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

            if (b1 <= b2 && b2 <= e1) || (b1 <= e2 && e2 <= e1) ||
               (b2 <= b1 && b1 <= e2) || (b2 <= e1 && e1 <= e2) {
               count += 1;
            }
         }
      }
   }
   
   count
}