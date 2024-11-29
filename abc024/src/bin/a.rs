#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        k:usize,
        s:usize,
        t:usize,
    }
    let mut ans = a * s + b * t;
    if k <= (s + t) {
        ans -= c * (s + t);
    }
    println!("{}", ans);
}
