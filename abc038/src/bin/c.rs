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
    let mut ans = n;
    let mut count = 0;
    for i in 1..n {
        if a[i - 1] < a[i] {
            count += 1;
        } else {
            ans += count * (count + 1) / 2;
            count = 0;
        }
    }
    ans += count * (count + 1) / 2;
    println!("{}", ans);
}
