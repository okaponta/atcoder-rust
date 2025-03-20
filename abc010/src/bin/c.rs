#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        s:(i64,i64),
        g:(i64,i64),
        t:usize,
        v:usize,
        n:usize,
        xy:[(i64,i64);n],
    }
    for xy in xy {
        if dist(s, xy) + dist(xy, g) <= ((t * v) as f64) {
            println!("YES");
            return;
        }
    }
    println!("NO");
}

fn dist(a: (i64, i64), b: (i64, i64)) -> f64 {
    (((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)) as f64).sqrt()
}
