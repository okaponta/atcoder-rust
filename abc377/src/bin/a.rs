#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut s:Chars,
    }
    s.sort();
    let s = s.iter().collect::<String>();
    println!("{}", if &s == "ABC" { "Yes" } else { "No" });
}
