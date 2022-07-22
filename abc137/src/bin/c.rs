use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut map = HashMap::new();
    for mut si in s {
        si.sort();
        *map.entry(si.iter().collect::<String>()).or_insert(0i64) += 1;
    }
    let ans = map.values().fold(0, |s, v| s + v * (v - 1) / 2);
    println!("{}", ans);
}
