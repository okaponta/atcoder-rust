#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:[usize;5],
    }
    let mut ans = vec![];
    for v in (0..5).combinations(3) {
        ans.push(v.into_iter().map(|i| n[i]).sum::<usize>());
    }
    ans.sort();
    ans.dedup();
    ans.reverse();
    println!("{}", ans[2]);
}
