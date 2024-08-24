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
        mut a:[usize;n],
    }
    a.rotate_right(k);
    println!("{}", a.iter().join(" "));
}
