use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {n:usize, k:usize}
    let mut set = HashSet::new();
    for _ in 0..k {
        input! {d:usize, a:[usize;d]}
        for e in a {
            set.insert(e);
        }
    }
    println!("{}", n - set.len());
}
