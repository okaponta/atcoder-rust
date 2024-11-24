use std::collections::HashMap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        s:Chars,
    }
    let a = (0..s.len() / 2).all(|i| s[2 * i] == s[2 * i + 1]);
    let mut map = HashMap::new();
    for c in s {
        *map.entry(c).or_insert(0) += 1;
    }
    let b = map.iter().all(|(_, &v)| v == 2);
    println!("{}", if a && b { "Yes" } else { "No" });
}
