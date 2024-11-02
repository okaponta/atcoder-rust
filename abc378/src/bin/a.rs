#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:[usize;4],
    }
    let mut count = vec![0; 5];
    for a in a {
        count[a] += 1;
    }
    println!("{}", count.iter().map(|i| i / 2).sum::<usize>());
}
