use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n:usize,
        s:Chars,
        q:usize,
        cd:[(char,char);q],
    }
    let mut map = HashMap::new();
    ('a'..='z').into_iter().for_each(|c| {
        map.insert(c, c);
    });
    for (c, d) in cd {
        ('a'..='z').into_iter().for_each(|b| {
            if map[&b] == c {
                map.insert(b, d);
            }
        });
    }
    println!("{}", s.into_iter().map(|c| map[&c]).join(""));
}
