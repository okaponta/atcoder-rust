use std::collections::BinaryHeap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        t:usize,
    }
    for _i in 0..t {
        solve();
    }
}

fn solve() {
    input! {n:usize,k:usize,mut a:[usize;n],b:[usize;n]}
    let ord = (0..n)
        .into_iter()
        .sorted_by_key(|&i| a[i])
        .collect::<Vec<_>>();
    let mut maxa = 0;
    let mut sumb = 0;
    let mut heap = BinaryHeap::new();
    for i in 0..k {
        maxa = a[ord[i]];
        sumb += b[ord[i]];
        heap.push(b[ord[i]]);
    }
    let mut ans = maxa * sumb;
    for i in k..n {
        maxa = a[ord[i]];
        if let Some(j) = heap.pop() {
            sumb -= j;
        }
        heap.push(b[ord[i]]);
        sumb += b[ord[i]];
        ans = ans.min(maxa * sumb)
    }
    println!("{ans}");
}
