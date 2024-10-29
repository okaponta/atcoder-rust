#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        d:usize,
    }
    let ao = a * d;
    let ta = b * c;
    let ans = if ao < ta {
        "TAKAHASHI"
    } else if ta < ao {
        "AOKI"
    } else {
        "DRAW"
    };
    println!("{}", ans);
}
