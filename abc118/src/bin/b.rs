use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        _:usize,
    }
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            k:usize,
            a:[usize;k],
        }
        for a in a {
            *map.entry(a).or_insert(0) += 1;
        }
    }
    println!("{}", map.values().filter(|&v| v == &n).count());
}
