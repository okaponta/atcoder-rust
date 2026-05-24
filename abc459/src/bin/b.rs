#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    s.into_iter()
        .map(|v| cv(v[0] as u8 - b'a'))
        .for_each(|i| print!("{i}"));
    println!();
}

fn cv(mut i: u8) -> u8 {
    if i < 18 {
        return 2 + i / 3;
    }
    i -= 1;
    if 2 + i / 3 < 9 {
        return 2 + i / 3;
    };
    9
}
