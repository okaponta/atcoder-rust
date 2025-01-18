#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:i64,b:i64,c:i64,
    }
    let mut q = 0;
    if a + b == c {
        q += 1;
    }
    if a - b == c {
        q += 2;
    }
    let ans = match q {
        0 => '!',
        1 => '+',
        2 => '-',
        _ => '?',
    };
    println!("{}", ans);
}
