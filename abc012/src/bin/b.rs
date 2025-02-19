#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut n:usize,
    }
    let mut min = n / 60;
    n %= 60;
    let hour = min / 60;
    min %= 60;
    println!("{:<02}:{:<02}:{:<02}", hour, min, n);
}
