#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut n:Chars,
    }
    n.dedup();
    println!("{}", if n.len() == 1 { "SAME" } else { "DIFFERENT" });
}
