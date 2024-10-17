#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    if n < 2 {
        println!("0");
        return;
    }
    let ans = (0..n - 2)
        .into_iter()
        .filter(|&i| s[i] == '#' && s[i + 1] == '.' && s[i + 2] == '#')
        .count();
    println!("{}", ans);
}
