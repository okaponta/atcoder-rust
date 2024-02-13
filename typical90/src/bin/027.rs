use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let mut set = HashSet::new();
    for i in 0..n {
        if !set.contains(&s[i]) {
            set.insert(&s[i]);
            println!("{}", i + 1);
        }
    }
}
