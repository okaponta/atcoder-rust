#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        se:[(usize,usize);3],
    }
    let ans = se.into_iter().map(|(s, e)| s * e / 10).sum::<usize>();
    println!("{}", ans);
}
