#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:usize,
        mut b:usize,
        c:usize,
    }
    if b < c {
        b += 24;
    }
    if c < a && a < b {
        println!("Yes");
        return;
    }
    if c < a + 24 && a + 24 < b {
        println!("Yes");
        return;
    }
    println!("No");
}
