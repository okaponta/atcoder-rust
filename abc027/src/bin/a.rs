#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut l:[usize;3],
    }
    l.sort();
    println!("{}", l[0] + l[2] - l[1]);
}
