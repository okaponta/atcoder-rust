use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut map = HashMap::new();
    for c in s {
        *map.entry(c).or_insert(0) += 1;
    }
    if *map.values().min().unwrap() == 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
