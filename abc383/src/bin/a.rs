#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        tv:[(usize,usize);n],
    }
    let mut vv = vec![0; tv[tv.len() - 1].0 + 1];
    for (t, v) in tv {
        vv[t] += v;
    }
    for i in 1..vv.len() {
        let tmp = vv[i - 1].saturating_sub(1) + vv[i];
        vv[i] = tmp;
    }
    println!("{}", vv[vv.len() - 1]);
}
