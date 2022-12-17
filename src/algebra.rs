use std::{cmp::Ordering, cmp::Eq, collections::HashMap};

#[derive(Debug, Eq, Clone, Hash, Copy)]
pub struct Point2D {
    pub x : i32,
    pub y : i32
}

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