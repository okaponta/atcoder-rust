#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:Chars,
        b:Chars,
    }
    let c = a
        .iter()
        .chain(b.iter())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("{}", c * 2);
}
