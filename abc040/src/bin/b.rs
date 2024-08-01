#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = n;
    for i in 1..=n {
        let j = n / i;
        let tmp = abs(i, j) + n - i * j;
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
