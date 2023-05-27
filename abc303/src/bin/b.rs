use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[[Usize1;n];m],
    }
    let mut set = HashSet::new();
    for i in 0..m {
        for j in 1..n {
            let k = a[i][j - 1];
            let l = a[i][j];
            set.insert((k.min(l), k.max(l)));
        }
    }
    println!("{}", n * (n - 1) / 2 - set.len());
}
