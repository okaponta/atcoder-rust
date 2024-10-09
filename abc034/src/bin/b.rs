#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut n:Usize1,
    }
    if n % 2 == 0 {
        n += 2;
    }
    println!("{}", n);
}
