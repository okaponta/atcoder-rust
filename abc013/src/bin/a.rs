#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        x:char,
    }
    println!("{}", x as u8 - b'A' + 1);
}
