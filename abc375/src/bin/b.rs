#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut xy:[(i64,i64);n],
    }
    xy.push((0, 0));
    xy.insert(0, (0, 0));
    let ans = (0..=n)
        .into_iter()
        .map(|i| dist(xy[i], xy[i + 1]))
        .sum::<f64>();
    println!("{}", ans);
}

fn dist((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> f64 {
    (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
}
