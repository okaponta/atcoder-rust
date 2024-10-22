use std::collections::BTreeSet;

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
        mut b:[usize;n-1],
    }
    let mut set = (0..n)
        .into_iter()
        .map(|i| (a[i], i))
        .collect::<BTreeSet<_>>();
    b.sort();
    for b in b {
        if let Some((a, i)) = set.range(..=(b, n)).last() {
            set.remove(&(*a, *i));
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", set.iter().next().unwrap().0);
}
