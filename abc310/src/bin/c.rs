use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut set = HashSet::new();
    for s in s {
        let s1 = s.iter().collect::<String>();
        let s2 = s.iter().rev().collect::<String>();
        if set.contains(&s1) || set.contains(&s2) {
            continue;
        }
        set.insert(s1);
    }
    println!("{}", set.len());
}
