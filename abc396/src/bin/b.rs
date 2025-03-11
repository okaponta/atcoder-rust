#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        q:usize,
    }
    let mut deck = vec![0; 100];
    for _ in 0..q {
        input! {qi: u8}
        if qi == 1 {
            input! {x: usize}
            deck.push(x);
        } else {
            println!("{}", deck.pop().unwrap());
        }
    }
}
