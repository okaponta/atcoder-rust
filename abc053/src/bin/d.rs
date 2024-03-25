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
    let mut c = 0;
    for (_, v) in map {
        c += v - 1;
    }
    println!("{}", n - ((c + 1) / 2) * 2);
}
