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
        c:usize,
    }
    let mut rank = vec![a, b, c];
    rank.sort();
    rank.reverse();
    println!("{}", rank.iter().position(|&i| i == a).unwrap() + 1);
    println!("{}", rank.iter().position(|&i| i == b).unwrap() + 1);
    println!("{}", rank.iter().position(|&i| i == c).unwrap() + 1);
}
