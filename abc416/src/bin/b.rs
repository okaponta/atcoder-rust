#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        mut s:Chars,
    }
    let mut flg = true;
    for i in 0..s.len() {
        if s[i] == '#' {
            flg = true;
        } else if flg {
            s[i] = 'o';
            flg = false;
        }
    }
    println!("{}", s.iter().join(""));
}
