#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut n:f64,
        m:f64,
    }
    if 11.5 < n {
        n -= 12.0;
    }
    let nd = 30.0 * n + 0.5 * m;
    let md = 6.0 * m;
    let d = md.max(nd) - md.min(nd);
    println!("{}", d.min(360.0 - d));
}
