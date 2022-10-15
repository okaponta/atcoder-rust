use std::collections::BTreeMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map = BTreeMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut ans = vec![0; n];
    for (i, (_, v)) in map.into_iter().rev().enumerate() {
        ans[i] = v;
    }
    for a in ans {
        println!("{}", a);
    }
}
