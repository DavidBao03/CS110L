/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut nv = Vec::new();

    for item in v.iter() {
        nv.push(item + n); 
    }

    nv
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for index in 0..v.len() {
        v[index] += n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut h = HashSet::new();
    let mut nv = Vec::new();

    for index in 0..v.len() {
        if h.contains(&v[index]) {
            continue;
        } else {
            h.insert(v[index]);
            nv.push(v[index]);
        }
    }

    *v = nv.clone();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
