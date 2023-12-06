use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut set = HashSet::new();
    let mut ast = 0;
    for a in a {
        if 7 < a / 400 {
            ast += 1;
        } else {
            set.insert(a / 400);
        }
    }
    println!("{} {}", set.len().max(1), set.len() + ast);
}
