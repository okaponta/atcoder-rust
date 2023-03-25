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
    let mut ans = 0usize;
    for (_, v) in map {
        ans += v / 2;
    }
    println!("{}", ans);
}
