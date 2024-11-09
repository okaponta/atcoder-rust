#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:Chars,
    }
    println!("{}{}{} {}{}{}", n[1], n[2], n[0], n[2], n[0], n[1]);
}
