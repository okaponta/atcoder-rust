#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = 0;
    for i in 0..n {
        ans += a.upper_bound(&(a[i] / 2));
    }
    println!("{}", ans);
}
