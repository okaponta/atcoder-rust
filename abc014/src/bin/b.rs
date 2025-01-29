#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut x:usize,
        a:[usize;n],
    }
    let mut ans = 0;
    for i in 0..n {
        if x & 1 == 1 {
            ans += a[i];
        }
        x >>= 1;
    }
    println!("{}", ans);
}
