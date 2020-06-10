use std::ops::{Index, IndexMut};

pub fn insert_at_zero<T>(vec: &mut Vec<T>, val: T) {
    let i = vec.len();
    vec.push(val);
    vec.swap(0, i);
}

pub struct Grid<T> {
    width: usize,
    elems: Vec<T>,
}
impl<T: Clone> Grid<T> {
    pub fn new(val: T, width: usize, height: usize) -> Grid<T> {
        Grid {
            width,
            elems: vec![val; width * height],
        }
    }
}
impl<T> Index<[usize; 2]> for Grid<T> {
    type Output = T;
    fn index(&self, [x, y]: [usize; 2]) -> &T {
        if x > self.width {
            panic!("x = {} out of range: {}", x, self.width);
        }
        &self.elems[x + y * self.width]
    }
}
impl<T> IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut T {
        if x > self.width {
            panic!("x = {} out of range: {}", x, self.width);
        }
        &mut self.elems[x + y * self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_zero_basic() {
        let mut v = vec![1, 2, 3];
        insert_at_zero(&mut v, 4);
        assert_eq!(v, vec![4, 2, 3, 1]);
    }
    #[test]
    fn insert_zero_multiple() {
        let mut v = vec!['a', 'b', 'c'];
        insert_at_zero(&mut v, 'd');
        assert_eq!(v, vec!['d', 'b', 'c', 'a']);
        insert_at_zero(&mut v, 'e');
        assert_eq!(v, vec!['e', 'b', 'c', 'a', 'd']);
        insert_at_zero(&mut v, 'f');
        assert_eq!(v, vec!['f', 'b', 'c', 'a', 'd', 'e']);
    }
    #[test]
    fn insert_zero_empty() {
        let mut v = vec![];
        insert_at_zero(&mut v, true);
        assert_eq!(v, vec![true])
    }

    #[test]
    fn grid_basic() {
        let g = Grid::new(0, 14, 10);
        assert_eq!(g[[3, 4]], 0);
    }
    #[test]
    fn grid_mut() {
        let mut g = Grid::new(0, 14, 10);
        g[[3, 4]] = 2;
        assert_eq!(g[[3, 4]], 2);
    }
    #[test]
    fn grid_step() {
        let g = Grid::new(true, 2, 20);
        assert!(g[[1, 19]]);
    }
    #[test]
    #[should_panic]
    fn grid_bounds() {
        let g = Grid::new(true, 2, 20);
        g[[19, 1]];
    }
}
