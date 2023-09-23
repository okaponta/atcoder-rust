use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
    }
    let mut map = HashMap::from([(0, 1)]);
    let mut s = 0;
    for a in a {
        s += a;
        *map.entry(s % m).or_insert(0usize) += 1;
    }
    let ans = map.into_iter().fold(0, |s, (_, v)| s + v * (v - 1) / 2);
    println!("{}", ans);
}
