use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut set = HashSet::new();
    let mut xy = (0, 0);
    set.insert(xy);
    for c in s {
        match c {
            'R' => xy.0 += 1,
            'L' => xy.0 -= 1,
            'U' => xy.1 += 1,
            _ => xy.1 -= 1,
        }
        set.insert(xy);
    }
    println!("{}", if set.len() == n + 1 { "No" } else { "Yes" });
}
