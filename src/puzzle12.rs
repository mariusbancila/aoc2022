use crate::utils;
use std::collections::VecDeque;
use std::{path::Path};
use std::fmt::{self};

struct Matrix<T> {
    rows : usize,
    cols : usize,
    data : Vec<T>
}

impl<T: Clone + Copy + std::cmp::PartialEq> Matrix<T> {
    fn new(r: usize, c: usize, init : T) -> Matrix<T> {
        let mut d = Vec::new();
        d.resize(r * c, init);

        Matrix {rows : r, cols : c, data : d}
    }

    fn element_at(&self, r: usize, c: usize) -> T {
        if r>= self.rows || c >= self.cols {
            panic!("Index out of bounds");
        }

        self.data[r*self.cols + c]
    }

    fn element_at_int(&self, r: i32, c: i32) -> T {
        let ur = usize::try_from(r).unwrap();
        let uc = usize::try_from(c).unwrap();

        if ur>= self.rows || uc >= self.cols {
            panic!("Index out of bounds");
        }

        self.data[ur*self.cols + uc]
    }    
    
    fn set_at(&mut self, r: usize, c: usize, value : T) {
        self.data[r*self.cols + c] = value;
    }
 
    fn set_at_int(&mut self, r: i32, c: i32, value : T) {
        let ur = usize::try_from(r).unwrap();
        let uc = usize::try_from(c).unwrap();

        self.data[ur*self.cols + uc] = value;
    }

    fn is_valid_position_int(&self, r: i32, c: i32) -> bool {
        r >= 0 && r < i32::try_from(self.rows).unwrap() && 
        c >= 0 && c < i32::try_from(self.cols).unwrap()
    }

    fn is_valid_position(&self, r: usize, c: usize) -> bool {
        r < self.rows && c < self.cols
    }

    fn find_char_position(&self, e : T) -> (usize, usize) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.element_at(r, c) == e {
                    return (r,c);
                }
            }
        }

        (usize::MAX, usize::MAX)
    }
}

type CharMatrix = Matrix<char>;
type NumericMatrix = Matrix<i32>;

impl<T: Clone + Copy + std::cmp::PartialEq + std::fmt::Display> fmt::Display for Matrix<T> {
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

fn from_file<P>(filename : P) -> CharMatrix
where P : AsRef<Path> {
    let mut r : usize = 0;
    let mut c : usize = 0;
    let mut d : Vec<char> = Vec::new();

    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(text) = line {
                let chars : Vec<char> = text.chars().collect();
                let row : Vec<char> = chars.into_iter().collect();

                if c == 0 {
                    c = row.len();
                }

                d.extend(row);

            }

            r += 1;
        }
    }

    CharMatrix {rows : r, cols : c, data : d}
}

const START_PLACE : char = 'S';
const END_PLACE : char = 'E';
const VISITED_PLACE : char = '*';
const UNVISITED_PLACE : char = '.';

fn is_end(matrix : &CharMatrix, r : usize, c: usize) -> bool {
    matrix.is_valid_position(r, c) && matrix.element_at(r, c) == END_PLACE
}

fn is_free(matrix : &CharMatrix, r : usize, c: usize) -> bool {
    matrix.is_valid_position(r, c) && matrix.element_at(r, c) == UNVISITED_PLACE
}

fn actual_hight(h : char) -> i32 {
    let n = match h {
        START_PLACE => 'a',
        END_PLACE => 'z',
        _   => h
    };

    i32::try_from(n as usize - 'a' as usize).unwrap()
}

// ========================= SOLUTION 1 : backtracking ========================= 

fn shortest_path_rec(
    matrix : &CharMatrix, 
    solution : &mut CharMatrix, 
    fromrow: usize, fromcol : usize, 
    torow : usize, tocol : usize, 
    xdir : &Vec<i32>, 
    ydir : &Vec<i32>, 
    dist : usize, 
    min_dist : &mut usize) {

    // end test
    if fromrow == torow && fromcol == tocol && is_end(matrix, fromrow, fromcol) {
        *min_dist = dist.min(*min_dist);

        return;
    }

    solution.set_at(fromrow, fromcol, VISITED_PLACE);

    for dir in 0..4 {
        let next_r = i32::try_from(fromrow).unwrap() + ydir[dir];
        let next_c = i32::try_from(fromcol).unwrap() + xdir[dir];

        if solution.is_valid_position_int(next_r, next_c) {
            let unext_r = usize::try_from(next_r).unwrap();
            let unext_c = usize::try_from(next_c).unwrap();

            let current_height = actual_hight(matrix.element_at(fromrow, fromcol));
            let next_height = actual_hight(matrix.element_at(unext_r, unext_c));

            if (current_height == next_height || current_height == next_height - 1) && is_free(&solution, unext_r, unext_c) {
                shortest_path_rec(matrix, solution, unext_r, unext_c, torow, tocol, xdir, ydir, dist + 1, min_dist);
            }
        }
    }

    solution.set_at(fromrow, fromcol, UNVISITED_PLACE);
}

fn shortest_path(matrix : &CharMatrix) -> usize {
    let mut solution = CharMatrix::new(matrix.rows, matrix.cols, UNVISITED_PLACE);
    // directions:   U  L  D  R
    let xdir = vec![ 0,-1, 0, 1];
    let ydir = vec![-1, 0, 1, 0];

    let (sr, sc) = matrix.find_char_position(START_PLACE);
    let (er, ec) = matrix.find_char_position(END_PLACE);

    let mut min_dist = usize::MAX;

    shortest_path_rec(matrix, &mut solution, sr, sc, er, ec, &xdir, &ydir, 0, &mut min_dist);    

    min_dist
}

// ========================= SOLUTION 2 : breath-first search ========================= 

struct Node {
    x : usize,
    y : usize,
    dist : usize
}

impl Node {
    fn new(x : usize, y :usize, d: usize) -> Node {
        Node {x : x, y : y, dist : d}
    }
}

fn shortest_path_bfs(matrix : &CharMatrix) -> usize {
    let mut solution = CharMatrix::new(matrix.rows, matrix.cols, UNVISITED_PLACE);

    // directions:   U  L  D  R
    let xdir = vec![ 0,-1, 0, 1];
    let ydir = vec![-1, 0, 1, 0];

    let (sr, sc) = matrix.find_char_position(START_PLACE);
    let (er, ec) = matrix.find_char_position(END_PLACE);

    solution.set_at(sr, sc, VISITED_PLACE);

    let mut queue : VecDeque<Node> = VecDeque::new();
    queue.push_back(Node::new(sr, sc, 0));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if current.x == ec && current.y == er {
            return current.dist;
        }
        
        for dir in 0..4 {
            let next_r = i32::try_from(current.y).unwrap() + ydir[dir];
            let next_c = i32::try_from(current.x).unwrap() + xdir[dir];
    
            if solution.is_valid_position_int(next_r, next_c) {
                let unext_r = usize::try_from(next_r).unwrap();
                let unext_c = usize::try_from(next_c).unwrap();
    
                let current_height = actual_hight(matrix.element_at(current.y, current.x));
                let next_height = actual_hight(matrix.element_at(unext_r, unext_c));
    
                if (current_height == next_height || current_height == next_height - 1) && is_free(&solution, unext_r, unext_c) {
                    solution.set_at(unext_r, unext_c, VISITED_PLACE);
                    queue.push_back(Node::new(unext_c, unext_r, current.dist + 1));
                }
            }            
        }
    }

    0
}

// ========================= SOLUTION 3 : reverse search ========================= 
fn next_step(matrix : &CharMatrix, solution : &mut NumericMatrix, dy: i32, dx: i32) -> usize {
    let height = i32::try_from(matrix.rows).unwrap();
    let width = i32::try_from(matrix.cols).unwrap();

    let starty = 0.max(-dy);
    let startx = 0.max(-dx);

    let mut change = 0;

    for y in starty..height {
        let next_y = y + dy;

        if next_y >= 0 && next_y < height {
            for x in startx..width {                
                let next_x = x + dx;
                
                if next_x >= 0 && next_x < width {
                    let current_height = actual_hight(matrix.element_at_int(y, x));
                    let next_height = actual_hight(matrix.element_at_int(next_y, next_x));

                    if current_height >= next_height-1 && solution.element_at_int(y, x)-1 > solution.element_at_int(next_y, next_x) {
                        solution.set_at_int(y, x, solution.element_at_int(next_y, next_x) + 1);
                        change += 1;
                    }
                }
                else {
                    break;
                }
            }
        }
        else {
            break;
        }
    }

    change
}

fn shortest_path_rev(matrix : &CharMatrix) -> i32 {
    let mut solution = NumericMatrix::new(matrix.rows, matrix.cols, i32::MAX);

    // directions:   U  L  D  R
    let xdir = vec![ 0,-1, 0, 1];
    let ydir = vec![-1, 0, 1, 0];

    let (sr, sc) = matrix.find_char_position(START_PLACE);
    let (er, ec) = matrix.find_char_position(END_PLACE);

    solution.set_at(er, ec, 0);

    loop {
        let mut change = 0;

        for dir in 0..4 {
            change += next_step(matrix, &mut solution, ydir[dir], xdir[dir]);
        }

        if change == 0 {
            break;
        }
    }
    
    solution.element_at(sr, sc)
}

fn any_shortest_path_rev(matrix : &CharMatrix) -> i32 {
    let mut solution = NumericMatrix::new(matrix.rows, matrix.cols, i32::MAX);

    // directions:   U  L  D  R
    let xdir = vec![ 0,-1, 0, 1];
    let ydir = vec![-1, 0, 1, 0];

    let (er, ec) = matrix.find_char_position(END_PLACE);

    solution.set_at(er, ec, 0);

    loop {
        let mut change = 0;

        for dir in 0..4 {
            change += next_step(matrix, &mut solution, ydir[dir], xdir[dir]);
        }

        if change == 0 {
            break;
        }
    }
    
    let mut distance = i32::MAX;
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            if actual_hight(matrix.element_at(r, c)) == 0 {
                if distance > solution.element_at(r, c) {
                    distance = solution.element_at(r, c);
                }
            }
        }
    }

    distance
    
}

pub fn execute() {
    println!("=== puzzle 12 ===");

    let test_matrix = from_file("./data/input12test.txt");
    let test_sp = shortest_path(&test_matrix);
    assert_eq!(31, test_sp);

    let test_sp_bfs = shortest_path_bfs(&test_matrix);
    assert_eq!(31, test_sp_bfs);

    let test_sp_rev = shortest_path_rev(&test_matrix);
    assert_eq!(31, test_sp_rev);

    let matrix = from_file("./data/input12.txt");
    let sp = shortest_path_rev(&matrix);
    println!("shortest path = {}", sp);

    let test_sp_rev = any_shortest_path_rev(&test_matrix);
    assert_eq!(29, test_sp_rev);

    let spa = any_shortest_path_rev(&matrix);
    println!("any shortest path = {}", spa);

    println!();
}