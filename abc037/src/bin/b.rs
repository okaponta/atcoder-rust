#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        lrt:[(Usize1,usize,usize);q],
    }
    let mut ans = vec![0; n];
    for (l, r, t) in lrt {
        for i in l..r {
            ans[i] = t;
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
