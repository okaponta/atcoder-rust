use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        s:Chars,
    }
    let map = vec!['a', 't', 'c', 'o', 'd', 'e', 'r']
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();
    let mut count = vec![0; 8];
    count[0] = 1usize;
    for c in s {
        if map.contains_key(&c) {
            count[map[&c]] += count[map[&c] - 1];
            count[map[&c]] %= 1_000_000_007;
        }
    }
    println!("{}", count[7]);
}
