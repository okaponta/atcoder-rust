#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut a:[usize;3],
    }
    a.sort();
    println!("{}", a[1]);
}
