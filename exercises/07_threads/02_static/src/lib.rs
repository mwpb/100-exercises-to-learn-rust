// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread::{self, JoinHandle};

pub fn sum(slice: &'static [i32]) -> i32 {
    let midpoint: usize = slice.len() / 2;
    let v1 = &slice[..midpoint];
    let v2 = &slice[midpoint..];

    let t1: JoinHandle<i32> = thread::spawn(|| v1.iter().sum());
    let t2: JoinHandle<i32> = thread::spawn(|| v2.iter().sum());

    let s1 = t1.join().unwrap();
    let s2 = t2.join().unwrap();

    s1 + s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
