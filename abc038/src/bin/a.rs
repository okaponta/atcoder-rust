#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut s:Chars,
    }
    let c = s.pop().unwrap();
    println!("{}", if c == 'T' { "YES" } else { "NO" });
}
