#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut a:[usize;4],
    }
    a.sort();
    a.dedup();
    println!("{}", if a.len() == 2 { "Yes" } else { "No" });
}
