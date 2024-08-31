#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[(i64,char);n],
    }
    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;
    for (a, s) in a {
        if s == 'L' {
            if l != 0 {
                ans += (a - l).abs();
            }
            l = a;
        } else {
            if r != 0 {
                ans += (a - r).abs();
            }
            r = a;
        }
    }
    println!("{}", ans);
}
