#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        d:usize,
        mut a:[usize;n],
    }
    if d == 0 {
        a.sort();
        a.dedup();
        println!("{}", n - a.len());
        return;
    }
    let mut count = vec![0; 2_500_000];
    let mut max = 0;
    for i in 0..n {
        count[a[i]] += 1;
        max = max.max(a[i]);
    }
    let mut ans = 0;
    for i in 0..d {
        let mut j = 0;
        let mut dp = vec![];
        for k in (i..=max).step_by(d) {
            if j == 0 {
                dp.push(0);
            } else if j == 1 {
                dp.push(count[k].min(count[k - d]));
            } else {
                let next = (dp[j - 1] + count[k]).min(dp[j - 2] + count[k - d]);
                dp.push(next);
            }
            j += 1;
        }
        if dp.len() != 0 {
            ans += dp[dp.len() - 1];
        }
    }
    println!("{}", ans);
}
