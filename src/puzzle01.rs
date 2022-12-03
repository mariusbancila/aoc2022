mod utils;

use std::path::Path;

fn main() {
   let calories = find_maximum("./data/input01.txt");
   println!("{}", calories);
   
   let (c1, c2, c3) = find_maximums("./data/input01.txt");
   println!("{},{},{} => {}", c1,c2,c3, c1+c2+c3);
}

fn find_maximum<P>(filename: P) -> i32
where P: AsRef<Path>, {
   let mut calories:i32 = 0;
   
   if let Ok(lines) = utils::read_lines(filename) {
      let mut total:i32 = 0;
      
      for line in lines {
            if let Ok(ip) = line {
               if ip.is_empty() {
                  if calories < total {
                     calories = total;
                  }
                  total = 0;
               }
               else {
                  let value = ip.parse::<i32>().unwrap();
                  total += value;
               }
            }
        }
   }
   
   calories
}

fn find_maximums<P>(filename: P) -> (i32, i32, i32)
where P: AsRef<Path>, {
   let mut c1:i32 = 0;
   let mut c2:i32 = 0;
   let mut c3:i32 = 0;
   
   if let Ok(lines) = utils::read_lines(filename) {
      let mut total:i32 = 0;
      
      for line in lines {
            if let Ok(ip) = line {
               if ip.is_empty() {
                  if c1 < total {
                     c3 = c2;
                     c2 = c1;
                     c1 = total;
                  }
                  else if c2 < total {
                     c3 = c2;
                     c2 = total;
                  }
                  else if c3 < total {
                     c3 = total;
                  }
                  total = 0;
               }
               else {
                  let value = ip.parse::<i32>().unwrap();
                  total += value;
               }
            }
        }
   }
   
   (c1, c2, c3)
}