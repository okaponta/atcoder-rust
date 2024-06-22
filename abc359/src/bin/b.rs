use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;2*n],
    }
    let mut map = HashMap::new();
    for i in 0..2 * n {
        map.entry(a[i]).or_insert(vec![]).push(i);
    }
    let mut ans = 0;
    for (_, v) in map {
        if abs(v[0], v[1]) == 2 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
