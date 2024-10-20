#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        k:usize,
        s:[usize;n],
    }
    if s.iter().any(|&i| i == 0) {
        println!("{n}");
        return;
    }
    let mut ans = 0;
    let mut tmp = 1;
    let mut r = 0;
    for l in 0..n {
        while r < n && tmp * s[r] <= k {
            tmp *= s[r];
            r += 1;
        }
        ans = ans.max(r - l);
        if l != r {
            tmp /= s[l];
        } else {
            r += 1;
        }
    }
    println!("{}", ans);
}
