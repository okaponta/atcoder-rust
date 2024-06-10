use std::collections::HashSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        k:usize,
    }
    let d = divisor(k);
    let mut set = HashSet::new();
    for &i in &d {
        for &j in &d {
            if k / i / j == 0 {
                continue;
            }
            if k % (i * j) != 0 {
                continue;
            }
            set.insert(sort(i, j, k / i / j));
        }
    }
    println!("{}", set.len());
}

fn sort(a: usize, b: usize, c: usize) -> (usize, usize, usize) {
    let mut v = vec![a, b, c];
    v.sort();
    (v[0], v[1], v[2])
}

fn divisor(n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut upper = vec![];
    for i in 1..=n.sqrt() {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                upper.push(n / i);
            }
        }
    }
    upper.reverse();
    res.append(&mut upper);
    res
}
