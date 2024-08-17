#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut x:Chars,
    }
    while let Some(c) = x.pop() {
        if c != '0' {
            x.push(c);
            break;
        }
    }
    if let Some(c) = x.pop() {
        if c != '.' {
            x.push(c);
        }
    }
    println!("{}", x.iter().join(""));
}
