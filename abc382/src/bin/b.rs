#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        _n:usize,
        d:usize,
        mut s:Chars,
    }
    for _i in 0..d {
        for j in (0..s.len()).rev() {
            if s[j] == '@' {
                s[j] = '.';
                break;
            }
        }
    }
    println!("{}", s.iter().join(""));
}
