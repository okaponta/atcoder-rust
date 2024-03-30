use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut set = HashSet::new();
    for i in 0..s.len() {
        let mut tmp = vec![];
        for j in i..s.len() {
            tmp.push(s[j]);
            set.insert(tmp.clone());
        }
    }
    println!("{}", set.len());
}
