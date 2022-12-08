use crate::utils;
use std::path::Path;
use std::fmt::{self};

struct Matrix {
    rows : usize,
    cols : usize,
    data : Vec<u32>
}

impl Matrix {
    fn from_file<P>(filename : P) -> Matrix 
    where P : AsRef<Path> {
        let mut r : usize = 0;
        let mut c : usize = 0;
        let mut d : Vec<u32> = Vec::new();

        if let Ok(lines) = utils::read_lines(filename) {
            for line in lines {
                if let Ok(text) = line {
                    let chars : Vec<char> = text.chars().collect();
                    let nums = chars.into_iter()
                                                .map(|c| c.to_digit(10))
                                                .collect::<Option<Vec<_>>>()
                                                .unwrap_or_default();

                    if c == 0 {
                        c = nums.len();
                    }

                    d.extend(nums);

                }

                r += 1;
            }
        }

        Matrix {rows : r, cols : c, data : d}
    }

    fn element_at(&self, r: usize, c: usize) -> u32 {
        if r>= self.rows || c >= self.cols {
            panic!("Index out of bounds");
        }

        self.data[r*self.cols + c]
    }    
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = writeln!(f, "matrix [{}x{}]", self.rows, self.cols);
        if let Err(_) = result {
            return result;
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                result = write!(f, "{} ", self.element_at(i, j));
                if let Err(_) = result {
                    return result;
                }
            }

            result = writeln!(f);
            if let Err(_) = result {
                return result;
            }
        }
        writeln!(f)
    }
}

fn find_visible_trees(mat : &Matrix) -> usize {
    let mut count = 2 * (mat.rows + mat.cols - 2);

    for r in 1..mat.rows-1 {
        for c in 1..mat.cols-1 {
            let h = mat.element_at(r, c);

            // up
            let mut visible = true;
            for rr in 0..r {
                if mat.element_at(rr, c) >= h {
                    visible = false;
                    break;
                }
            }

            if visible {
                count += 1;
                continue;
            }
            
            // down
            visible = true;
            for rr in r+1..mat.rows {
                if mat.element_at(rr, c) >= h {
                    visible = false;
                    break;
                }
            }

            if visible {
                count += 1;
                continue;
            }

            // left
            visible = true;
            for cc in 0..c {
                if mat.element_at(r, cc) >= h {
                    visible = false;
                    break;
                }
            }

            if visible {
                count += 1;
                continue;
            }

            // right
            visible = true;
            for cc in c+1..mat.cols {
                if mat.element_at(r, cc) >= h {
                    visible = false;
                    break;
                }
            }

            if visible {
                count += 1;
                continue;
            }
        }
    }

    count
}

fn find_maximum_scenic_score(mat: &Matrix) -> u32 {
    let mut max_score : u32 = 0;

    for r in 1..mat.rows-1 {
        for c in 1..mat.cols-1 {
            let h = mat.element_at(r, c);

            // up
            let mut score_up = 0;
            for rr in (0..r).rev() {
                let value = mat.element_at(rr, c);
                if value < h {
                    score_up += 1;
                }
                else if value >= h {
                    score_up += 1;
                    break;
                }
            }

            // down
            let mut score_down = 0;
            for rr in r+1..mat.rows {
                let value = mat.element_at(rr, c);
                if value < h {
                    score_down += 1;
                }
                else if value >= h {
                    score_down += 1;
                    break;
                }
            }

            // left
            let mut score_left = 0;
            for cc in (0..c).rev() {
                let value = mat.element_at(r, cc);
                if value < h {
                    score_left += 1;
                }
                else if value >= h {
                    score_left += 1;
                    break;
                }
            }

            // right
            let mut score_right = 0;
            for cc in c+1..mat.cols {
                let value = mat.element_at(r, cc);
                if value < h {
                    score_right += 1;
                }
                else if value >= h {
                    score_right += 1;
                    break;
                }
            }
            let score = score_up * score_down * score_left * score_right;

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

pub fn execute() {
    println!("=== puzzle 8 ===");

    let mtest = Matrix::from_file("./data/input08test.txt");
    print!("{}", mtest);
    let visible_test = find_visible_trees(&mtest);
    assert_eq!(21, visible_test);
    let scenic_score_test = find_maximum_scenic_score(&mtest);
    assert_eq!(8, scenic_score_test);

    let m = Matrix::from_file("./data/input08.txt");
    let visible = find_visible_trees(&m);
    println!("visible={}", visible);
    let scenic_score = find_maximum_scenic_score(&m);
    println!("score={}", scenic_score);

    println!();
}