use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut set = HashSet::new();
    for c in s {
        set.insert(c.to_digit(10).unwrap() as i32);
    }
    for i in 0..10 {
        if set.contains(&i) {
            continue;
        } else {
            println!("{}", i);
        }
    }
}
