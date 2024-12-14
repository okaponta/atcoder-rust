#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        c1:char,
        c2:char,
        mut s:Chars,
    }
    for i in 0..n {
        if s[i] != c1 {
            s[i] = c2;
        }
    }
    println!("{}", s.iter().join(""));
}
