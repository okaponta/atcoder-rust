#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        x:usize,
    }
    let mut ans = 0;
    for i in 1..10 {
        for j in 1..10 {
            if i * j != x {
                ans += i * j;
            }
        }
    }
    println!("{}", ans);
}
