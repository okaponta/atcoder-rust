#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        r:f64,
    }
    if r < 1.5 {
        println!("1");
        return;
    }
    let mut ans = 2 * f(r, 0.5).floor() as usize - 1;
    let mut dist = 1.5;
    while 1.0 < f(r, dist) {
        ans += 2 * (2 * f(r, dist).floor() as usize - 1);
        dist += 1.0;
    }
    println!("{}", ans);
}

fn f(r: f64, d: f64) -> f64 {
    0.5 + (r * r - d * d).sqrt()
}
