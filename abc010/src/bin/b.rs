#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = 0;
    for mut a in a {
        while a % 2 == 0 || a % 3 == 2 {
            a -= 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}
