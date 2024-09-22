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
    let ans = s.into_iter().filter(|&c| c != '.').collect::<String>();
    println!("{}", ans);
}
