// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread::{self, JoinHandle};

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let vleak = v.leak();

    let (v1, v2) = vleak.split_at(mid);

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
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
