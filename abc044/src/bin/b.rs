use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        w:Chars,
    }
    let mut map = HashMap::new();
    for c in w {
        *map.entry(c).or_insert(0) += 1;
    }
    if map.into_iter().all(|(_, v)| v % 2 == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
