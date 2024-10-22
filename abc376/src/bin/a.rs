#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        c:usize,
        t:[usize;n],
    }
    let mut ans = 1;
    let mut prev = t[0];
    for i in 1..n {
        if prev + c <= t[i] {
            prev = t[i];
            ans += 1;
        }
    }
    println!("{}", ans);
}
