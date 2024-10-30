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
        x:usize,
        y:usize,
        a:[usize;n],
        b:[usize;m],
    }
    let mut t = 0;
    let mut ans = 0;
    loop {
        let pa = a.lower_bound(&t);
        if pa == n {
            break;
        }
        t = a[pa] + x;
        let pb = b.lower_bound(&t);
        if pb == m {
            break;
        }
        t = b[pb] + y;
        ans += 1;
    }
    println!("{}", ans);
}
