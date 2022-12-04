use crate::utils;
use std::path::Path;

pub fn execute() {
   println!("=== puzzle 2 ===");
   
   let total = compute_total("./data/input02.txt");
   println!("total={}", total);
   
   let total = compute_correct_total("./data/input02.txt");
   println!("correct total={}", total);
   
   println!();
}

fn compute_total<P>(filename: P) -> i32
where P: AsRef<Path>, {
   let mut total : i32 = 0;
   
   if let Ok(lines) = utils::read_lines(filename) {
      for line in lines {
         if let Ok(ip) = line {
            let mut chs = ip.chars();
            let f :char = chs.nth(0).unwrap();
            let s :char = chs.nth(1).unwrap();
            
            total += round_score(f, s) + shape_score(s);
         }
      }
   }
   
   total
}

fn compute_correct_total<P>(filename: P) -> i32
where P: AsRef<Path>, {
   let mut total : i32 = 0;
   
   if let Ok(lines) = utils::read_lines(filename) {
      for line in lines {
         if let Ok(ip) = line {
            let mut chs = ip.chars();
            let s :char = chs.nth(0).unwrap();
            let r :char = chs.nth(1).unwrap();
            
            let points = match  r{
               'X' => 0, // lose
               'Y' => 3, // draw
               'Z' => 6, // win
               _ => panic!("Invalid choice!")
            };
            
            let c = get_shape_choice(s, r);
            let score = shape_score(c);
            
            total += points + score;
         }
      }
   }
   
   total
}



// A = Rock, B = Paper, C = Scissors
// X = Rock, Y = Paper, Z = Scissors
// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
// Score: 0 if you lost, 3 if the round was a draw, and 6 if you won
fn round_score(f : char, s : char) -> i32 {
   if (f == 'A' && s == 'Z') || (f == 'C' && s == 'Y') || (f == 'B' && s == 'X') {
      0
   }
   else if (s == 'X' && f == 'C') || (s == 'Z' && f == 'B') || (s == 'Y' && f == 'A') {
      6
   }
   else {
      3
   }
}

fn get_shape_choice(s: char, result : char) -> char {
   let result = match s {
      'A' => match result {
         'X' => 'C', // lose
         'Y' => 'A', // draw
         'Z' => 'B', // win
         _   => panic!("Invalid choice!")
      },
      'B' => match result {
         'X' => 'A', // lose
         'Y' => 'B', // draw
         'Z' => 'C', // win
         _   => panic!("Invalid choice!")
      },
      'C' => match result {
         'X' => 'B', // lose
         'Y' => 'C', // draw
         'Z' => 'A', // win
         _   => panic!("Invalid choice!")
      },
      _   => panic!("Invalid choice!")
   };
   
   result
}

// score: 1 for Rock, 2 for Paper, and 3 for Scissors
fn shape_score(s : char) -> i32 {
   let score = match s {
      'X'|'A' => 1,
      'Y'|'B' => 2,
      'Z'|'C' => 3,
      _ => panic!("Invalid choice!")
   };
   
   score
}