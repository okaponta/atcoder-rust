#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
    n:usize,
    a:[i64;n],
    }
    let mut ans = n;
    let mut bef = 1 << 60;
    let mut count = 0;
    for i in 1..n {
        let diff = a[i] - a[i - 1];
        if bef == diff {
            count += 1;
        } else {
            ans += count * (count + 1) / 2;
            bef = diff;
            count = 1;
        }
    }
    ans += count * (count + 1) / 2;
    println!("{}", ans);
}
