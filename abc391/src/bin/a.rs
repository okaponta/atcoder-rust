#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        s:Chars,
    }
    s.iter().for_each(|&c| print!("{}", f(c)));
    println!();
}

fn f(c: char) -> char {
    if c == 'N' {
        return 'S';
    }
    if c == 'S' {
        return 'N';
    }
    if c == 'E' {
        return 'W';
    }
    'E'
}
