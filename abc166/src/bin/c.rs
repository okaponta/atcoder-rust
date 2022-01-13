use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,
       h:[i32;n],
       ab:[(Usize1,Usize1);m]
    }
    let mut bad = HashSet::new();
    for (a, b) in ab {
        if h[a] > h[b] {
            bad.insert(b);
        } else if h[a] == h[b] {
            bad.insert(a);
            bad.insert(b);
        } else {
            bad.insert(a);
        }
    }
    println!("{}", n - bad.len());
}
