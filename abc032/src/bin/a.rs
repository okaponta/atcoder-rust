#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:usize,
        b:usize,
        mut n:usize,
    }
    while !(n % a == 0 && n % b == 0) {
        n += 1;
    }
    println!("{}", n);
}
