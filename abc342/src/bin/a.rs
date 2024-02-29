use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut map = HashMap::new();
    for i in 0..s.len() {
        map.entry(s[i]).or_insert(vec![]).push(i + 1);
    }
    for (_k, v) in map {
        if v.len() == 1 {
            println!("{}", v[0]);
        }
    }
}
