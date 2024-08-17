use std::collections::VecDeque;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
    }
    let mut dp = vec![0; m];
    let mut s = 0;
    let mut q = VecDeque::new();
    let mut ans = 0usize;
    for i in 0..n - 1 {
        s += a[i];
        dp[s % m] += 1;
        q.push_back(s % m);
    }
    let mut s2 = 0;
    for i in 0..n {
        ans += dp[s2 % m];
        s += a[(n + i - 1) % n];
        q.push_back(s % m);
        dp[s % m] += 1;
        s2 += a[i];
        dp[q.pop_front().unwrap()] -= 1;
    }
    println!("{}", ans);
}
