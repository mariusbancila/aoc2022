use crate::utils;
use std::path::Path;
use std::collections::HashSet;

pub fn execute() {
   println!("=== puzzle 3 ===");
   
   assert_eq!(1, get_priority('a'));
   assert_eq!(26, get_priority('z'));
   assert_eq!(27, get_priority('A'));
   assert_eq!(52, get_priority('Z'));
   
   let total_test = get_priorties_sum("./data/input03test.txt");
   assert_eq!(157, total_test);

   let total = get_priorties_sum("./data/input03.txt");
   println!("total={}", total);
   
   let total_test2 = get_priorties_sum2("./data/input03test.txt");
   assert_eq!(70, total_test2);
   
   let total2 = get_priorties_sum2("./data/input03.txt");
   println!("total2={}", total2);
   
   println!();
}

fn get_priority(item : char) -> i32 {
   match item {
      'a'..='z' => 1 + (item as i32 - 'a' as i32),
      'A'..='Z' => 27 + (item as i32 - 'A' as i32),
      _         => panic!("invalid item")
   }
}

fn get_priorties_sum<P>(filename : P) -> i32
where P: AsRef<Path>, {
   let mut total : i32 = 0;
   
   if let Ok(lines) = utils::read_lines(filename) {
      for line in lines {
         if let Ok(ip) = line {
            let chs = ip.chars();
            let size = chs.count() / 2;
            
            let left = &ip[..size];
            let right = &ip[size..size*2];
            
            let lset: HashSet<char> = left.chars().collect();
            let rset: HashSet<char> = right.chars().collect();
            
            let common: HashSet<char> = &lset & &rset;
            
            let item : char = common.iter().next().unwrap().clone();
            
            total += get_priority(item);
         }
      }
   }
   
   total
}

fn get_priorties_sum2<P>(filename : P) -> i32
where P: AsRef<Path>, {
   let mut total : i32 = 0;
   
   if let Ok(lines) = utils::read_lines(filename) {
      let mut count = 0;
      let mut set1 : HashSet::<char> = HashSet::<char>::new();
      let mut set2 : HashSet::<char> = HashSet::<char>::new();
      let mut set3;
      
      for line in lines {
         if let Ok(ip) = line {
            if count == 0 {
               set1 = ip.chars().collect();
               count+=1;
            }
            else if count == 1 {
               set2 = ip.chars().collect();
               count+=1;
            }
            else if count == 2 {
               set3 = ip.chars().collect();
               
               let common1: HashSet<char> = &set1 & &set2;
               let common2: HashSet<char> = &common1 & &set3;
            
               let item : char = common2.iter().next().unwrap().clone();
            
               total += get_priority(item);
               
               count = 0;
            }
         }
      }
   }
   
   total
}