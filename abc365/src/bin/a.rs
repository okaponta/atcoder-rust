#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
    }
    if n % 4 != 0 {
        println!("365");
    } else if n % 100 != 0 {
        println!("366");
    } else if n % 400 != 0 {
        println!("365");
    } else {
        println!("366");
    }
}
