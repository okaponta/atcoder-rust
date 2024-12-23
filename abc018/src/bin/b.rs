#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut s:Chars,
        n:usize,
        lr:[(Usize1,usize);n],
    }
    let m = s.len();
    for (l, r) in lr {
        s = (0..l)
            .into_iter()
            .chain((l..r).rev().into_iter())
            .chain((r..m).into_iter())
            .map(|i| s[i])
            .collect::<Vec<_>>();
    }
    println!("{}", s.iter().join(""));
}
