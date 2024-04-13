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
    let mut count = vec![0; 200];
    for (_, v) in map {
        count[v] += 1;
    }
    if count.into_iter().all(|i| i == 0 || i == 2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
