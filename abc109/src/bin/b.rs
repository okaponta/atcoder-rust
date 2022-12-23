use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        w:[Chars;n],
    }
    let mut set = HashSet::new();
    set.insert(w[0].iter().collect::<String>());
    for i in 1..n {
        if w[i][0] != *w[i - 1].last().unwrap() {
            println!("No");
            return;
        }
        set.insert(w[i].iter().collect::<String>());
    }
    println!("{}", if set.len() == n { "Yes" } else { "No" });
}
