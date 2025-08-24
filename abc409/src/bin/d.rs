#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        case();
    }
}

fn case() {
    input! {n:usize,mut s:Chars}
    if n == 1 {
        println!("{}", s[0]);
        return;
    }
    let mut i = 0;
    while i + 1 < n && s[i] <= s[i + 1] {
        i += 1;
    }
    let mut j = i + 1;
    while j < n && s[j] <= s[i] {
        j += 1;
    }
    s[i..j].rotate_left(1);
    println!("{}", s.iter().join(""))
}
