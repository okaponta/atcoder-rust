use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
        b:[Usize1;k],
    }
    let max = *a.iter().max().unwrap();
    let mut set = HashSet::new();
    for i in 0..n {
        if a[i] == max {
            set.insert(i);
        }
    }
    let is_ok = b.iter().filter(|&i| set.contains(i)).count() > 0;
    println!("{}", if is_ok { "Yes" } else { "No" });
}
