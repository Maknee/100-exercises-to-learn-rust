// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let slice: &'static mut [i32] = v.leak();
    let l = slice.len() / 2;
    let l1 = &slice[l..];
    let l2 = &slice[..l];
    let h1 = thread::spawn(move || {
        l1.iter().sum()
    });
    let h2 = thread::spawn(move || {
        l2.iter().sum()
    });

    let r1: i32 = h1.join().unwrap();
    let r2: i32 = h2.join().unwrap();
    r1 + r2    
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
