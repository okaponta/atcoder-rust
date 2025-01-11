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
    let mut d = vec![0; n + 1];
    let mut tmp = 0;
    let mut ans = vec![];
    for i in 0..n {
        tmp += d[i];
        let range = ((a[i] + tmp) as usize).min(n - 1 - i);
        ans.push((a[i] + tmp) as usize - range);
        d[i + 1] += 1;
        d[i + 1 + range] -= 1;
    }
    println!("{}", ans.iter().join(" "));
}
