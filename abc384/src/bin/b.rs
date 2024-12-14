#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut r:i64,
        da:[(u8,i64);n],
    }
    for (d, a) in da {
        if d == 1 {
            if 1600 <= r && r < 2800 {
                r += a;
            }
        } else {
            if 1200 <= r && r < 2400 {
                r += a;
            }
        }
    }
    println!("{}", r);
}
