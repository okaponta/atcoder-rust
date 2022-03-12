use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        b:[usize;n],
    }
    let mut hit = 0;
    for i in 0..n {
        if a[i] == b[i] {
            hit += 1;
        }
    }
    let mut blow = 0;
    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(a[i]);
    }
    for bi in b {
        if set.contains(&bi) {
            blow += 1;
        }
    }
    blow -= hit;
    println!("{}", hit);
    println!("{}", blow);
}
