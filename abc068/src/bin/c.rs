use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut set = HashSet::new();
    let mut rev_set = HashSet::new();
    for (a, b) in ab {
        if a.min(b) == 0 {
            set.insert(a.max(b));
        }
        if a.max(b) == n - 1 {
            rev_set.insert(a.min(b));
        }
    }
    for e in set {
        if rev_set.contains(&e) {
            println!("POSSIBLE");
            return;
        }
    }
    println!("IMPOSSIBLE");
}
