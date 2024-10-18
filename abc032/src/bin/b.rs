use proconio::{marker::*, *};
use std::collections::*;

fn main() {
    input! {s:Chars,k:usize}
    if s.len() < k {
        println!("0");
        return;
    }
    let mut set = HashSet::new();
    for i in 0..=s.len() - k {
        set.insert((0..k).into_iter().map(|j| s[i + j]).collect::<String>());
    }
    println!("{}", set.len());
}
