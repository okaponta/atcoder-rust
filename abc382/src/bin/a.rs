#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        _n:usize,
        d:usize,
        s:Chars,
    }
    let mut ans = s.iter().filter(|c| **c == '.').count();
    ans += d;
    println!("{}", ans);
}
