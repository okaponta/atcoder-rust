use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut set = HashSet::new();
    for c in s {
        set.insert(c);
    }
    for i in 0..26 {
        let c = (b'a' + i) as char;
        if !set.contains(&c) {
            println!("{}", c);
            return;
        }
    }
    println!("None");
}
