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
    let ans = s[0].to_digit(10).unwrap() as usize * s[2].to_digit(10).unwrap() as usize;
    println!("{}", ans);
}
