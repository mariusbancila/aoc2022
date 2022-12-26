use std::{cmp::Ordering, cmp::Eq, collections::HashMap};
use std::hash::Hash;

#[allow(unused)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Hash, Copy)]
pub struct Position2D<T> {
    pub x : T,
    pub y : T
}

#[allow(unused)]
impl<T> Position2D<T> {
    pub fn new(x : T, y : T) -> Position2D<T> {
        Position2D { x, y }
    }
}

#[allow(unused)]
#[derive(Debug, Eq, Clone, Hash, Copy)]
pub struct Point2D {
    pub x : i32,
    pub y : i32
}

#[allow(unused)]
impl Point2D {
    pub fn new(x : i32, y : i32) -> Point2D {
        Point2D { x, y }
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        }
        else if self.x == other.x {
            if self.y < other.y {
                Ordering::Less
            }
            else if self.y == other.y {
                Ordering::Equal
            }
            else {
                Ordering::Greater
            }
        }
        else {
            Ordering::Greater
        }
    }
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[allow(unused)]
#[derive(Debug, Eq, Clone, Hash, Copy)]
pub struct Point2DAlt {
    pub x : i32,
    pub y : i32
}

impl Point2DAlt {
    #[allow(unused)]
    pub fn new(x : i32, y : i32) -> Point2DAlt {
        Point2DAlt { x, y }
    }
}

impl PartialOrd for Point2DAlt {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2DAlt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        }
        else if self.y == other.y {
            if self.x < other.x {
                Ordering::Less
            }
            else if self.x == other.x {
                Ordering::Equal
            }
            else {
                Ordering::Greater
            }
        }
        else {
            Ordering::Greater
        }
    }
}

impl PartialEq for Point2DAlt {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[allow(unused)]
#[derive(Clone)]
pub struct SparseMatrix<T> {
    pub points : HashMap<Point2D, T>,
    pub left_most  : i32,
    pub right_most : i32,
    pub top_most : i32,
    pub bottom_most : i32,
}

#[allow(unused)]
impl<T> SparseMatrix<T> 
where T : PartialEq + Copy {
    pub fn new() -> SparseMatrix<T> {
        SparseMatrix {points : HashMap::new(), left_most : i32::MAX, right_most : 0, top_most : i32::MAX, bottom_most : 0}
    }

    pub fn from(p : &HashMap<Point2D, T>) -> SparseMatrix<T> {
        SparseMatrix {points : p.clone(), left_most : i32::MAX, right_most : 0, top_most : i32::MAX, bottom_most : 0}
    }

    pub fn element_at(&self, x: i32, y : i32) -> Option<T> {
        let pt = Point2D::new(x, y);

        if self.points.contains_key(&pt) {
            if let Some(c) = self.points.get(&pt) {
                return Some(*c);
            }
        }

        None
    }

    pub fn insert(&mut self, x : i32, y: i32, value : T) {
        let pt = Point2D::new(x, y);

        if self.points.contains_key(&pt) {
            if let Some(c) = self.points.get_mut(&pt) {
                *c = value;
                return;
            }
        }

        self.points.insert(pt, value);
    }

    pub fn try_insert(&mut self, x : i32, y: i32, value : T) -> bool {
        let pt = Point2D::new(x, y);

        if self.points.contains_key(&pt) {
            if let Some(c) = self.points.get(&pt) {
                if *c != value {
                    return false;
                }
            }
        }

        self.points.insert(pt, value);        
        true
    }
}

#[allow(unused)]
#[derive(Clone)]
pub struct SparseMatrixAlt<K, T> {
    pub points : HashMap<K, T>,
    pub left_most  : i32,
    pub right_most : i32,
    pub top_most : i32,
    pub bottom_most : i32,
}

#[allow(unused)]
impl<K, T> SparseMatrixAlt<K, T> 
where T : PartialEq + Copy, K : Copy + Eq + Hash {
    pub fn new() -> SparseMatrixAlt<K, T> {
        SparseMatrixAlt {points : HashMap::new(), left_most : i32::MAX, right_most : 0, top_most : i32::MAX, bottom_most : 0}
    }

    pub fn element_at(&self, key : &K) -> Option<T> {
        if self.points.contains_key(key) {
            if let Some(c) = self.points.get(key) {
                return Some(*c);
            }
        }

        None
    }

    pub fn insert(&mut self, key : &K, value : T) {
        if self.points.contains_key(key) {
            if let Some(c) = self.points.get_mut(key) {
                *c = value;
                return;
            }
        }

        self.points.insert(*key, value);
    }

    pub fn try_insert(&mut self, key : &K, value : T) -> bool {
        if self.points.contains_key(key) {
            if let Some(c) = self.points.get(key) {
                if *c != value {
                    return false;
                }
            }
        }

        self.points.insert(*key, value);
        true
    }
}

#[allow(unused)]
#[derive(Clone)]
pub struct Matrix<T> {
    pub rows : usize,
    pub cols : usize,
    pub data : Vec<T>
}

#[allow(unused)]
impl<T> Matrix<T> 
where T : Copy{
    pub fn new(r: usize, c : usize) -> Matrix<T> {
        Matrix { rows: r, cols: c, data: Vec::new() }
    }

    pub fn new_from(r: usize, c : usize, d: Vec<T>) -> Matrix<T> {
        Matrix { rows: r, cols: c, data: d }
    }

    pub fn element_at(&self, r : usize, c : usize) -> Option<T> {
        if r < self.rows && c < self.cols {
            return Some(self.data[r * self.cols + c]);
        }
        None
    }

    pub fn set_at(&mut self, r : usize, c : usize, value : T) -> bool {
        if r < self.rows && c < self.cols {
            self.data[r * self.cols + c] = value;
            return true;
        }

        false
    }
}

#[allow(unused)]
pub struct Matrix3D<T> {
    pub size_x : usize,
    pub size_y : usize,
    pub size_z : usize,
    pub data : Vec<Vec<Vec<T>>>
}

#[allow(unused)]
impl<T> Matrix3D<T> 
where T: Clone + Copy{
    pub fn new(sx : usize, sy : usize, sz : usize, value : T) -> Matrix3D<T> {
        Matrix3D { size_x: sx, size_y: sy, size_z: sz, data: vec![vec![vec![value; sz]; sy]; sz] }
    }

    pub fn element_at(&self, x : usize, y : usize, z : usize) -> Option<T> {
        if x < self.size_x && y < self.size_y && z < self.size_z {
            return Some(self.data[x][y][z]);
        }
        println!("{},{},{}", x,y,z);
        None
    }

    pub fn set_at(&mut self, x : usize, y : usize, z : usize, value : T) -> bool {
        if x < self.size_x && y < self.size_y && z < self.size_z {
            self.data[x][y][z] = value;
            return true;
        }
        false
    }
}