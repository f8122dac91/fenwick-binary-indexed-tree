//! # Fenwick binary indexed tree
//!
//! 1-indexed array that can:
//!   1. add an integer to an item with index
//!   2. return the sum of the items up to an index
//!
//! Both of these operations run in O(log n).
use std::ops::AddAssign;
use std::default::Default;

// TODO: generalize
#[derive(Debug)]
struct Ftree<T> {
    n: i64,
    a: Vec<T>,
}

impl <T> Ftree<T>
where
    T: Copy + AddAssign + Default
{
    fn new(n: i64) -> Self {
        Ftree {
            n,
            a: vec![Default::default(); n as usize + 1],
        }
    }

    /// add x to index
    fn add(&mut self, idx: i64, x: T) {
        assert!(idx> 0);
        let mut idx = idx;
        while idx <= self.n {
            self.a[idx as usize] += x;
            idx += idx & (-idx);
        }
    }

    /// return the sum of items in range [1, idx]
    fn query(&self, idx: i64) -> T {
        let mut sum = Default::default();
        let mut idx = idx;
        while idx > 0 {
            sum += self.a[idx as usize];
            idx -= idx & (-idx);
        }
        sum
    }
}

fn main() {
    let mut f: Ftree<i64> = Ftree::new(10);
    f.add(1, 1);
    f.add(2, 3);
    f.add(3, 8);
    f.add(3, 2);
    println!("f: {:?}", f);
    println!("f.query(3): {}", f.query(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let f: Ftree<i64> = Ftree::new(0);
        assert_eq!(f.query(0), 0);
    }

    #[test]
    fn it_works_3() {
        let mut f: Ftree<i64> = Ftree::new(3);
        f.add(1, 3);
        f.add(2, 14);
        f.add(3, 5);
        assert_eq!(f.query(3), 22);
    }

    #[test]
    fn it_works_10() {
        let mut f: Ftree<i64> = Ftree::new(10);
        f.add(1, 1);
        f.add(2, 1);
        f.add(3, 1);
        f.add(4, 1);
        f.add(5, 1);
        f.add(6, 1);
        f.add(7, 1);
        f.add(8, 1);
        f.add(9, 1);
        f.add(10, 1);
        assert_eq!(f.query(1), 1);
        assert_eq!(f.query(2), 2);
        assert_eq!(f.query(3), 3);
        assert_eq!(f.query(9), 9);
        assert_eq!(f.query(10), 10);
    }

    #[test]
    fn subtraction() {
        let mut f: Ftree<i64> = Ftree::new(2);
        f.add(1, 3);
        f.add(2, 14);
        assert_eq!(f.query(1), 3);
        assert_eq!(f.query(2), 17);
        f.add(1, -3);
        assert_eq!(f.query(1), 0);
        assert_eq!(f.query(2), 14);
        f.add(1, -3);
        assert_eq!(f.query(1), -3);
        assert_eq!(f.query(2), 11);
    }

    #[test]
    fn float_works() {
        let mut f: Ftree<f32> = Ftree::new(2);
        f.add(1, 3f32);
        f.add(2, 14f32);
        assert_eq!(f.query(1), 3f32);
        assert_eq!(f.query(2), 17f32);
    }
}
