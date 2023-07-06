use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        k:Usize1,
        a:[usize;n],
    }
    let mut map = HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut count = map.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    count.sort();
    count.reverse();
    let mut ans = 0;
    for i in 0..count.len() {
        if k < i {
            ans += count[i];
        }
    }
    println!("{}", ans);
}
