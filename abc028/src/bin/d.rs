#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    println!(
        "{}",
        (1 + 3 * (n - 1) + 6 * (n - k) * (k - 1)) as f64 / (n * n * n) as f64
    );
}
