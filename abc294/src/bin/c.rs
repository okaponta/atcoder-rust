use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        b:[usize;m],
    }
    let mut c = a.iter().chain(b.iter()).collect::<Vec<_>>();
    c.sort();

    let mut map = HashMap::new();
    for i in 0..n + m {
        map.insert(c[i], i + 1);
    }

    println!("{}", a.iter().map(|i| map[&i]).join(" "));
    println!("{}", b.iter().map(|i| map[&i]).join(" "));
}
