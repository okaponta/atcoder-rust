use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map = HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }
    let ans = map.into_iter().fold(0, |s, (k, v)| s + calc(k, v));
    println!("{}", ans);
}

fn calc(k: usize, v: usize) -> usize {
    if v < k {
        v
    } else {
        v - k
    }
}
