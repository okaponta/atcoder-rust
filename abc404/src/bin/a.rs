#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:Chars,
    }
    for i in 0..26 {
        let c = (b'a' + i) as char;
        if !s.contains(&c) {
            println!("{}", c);
            return;
        }
    }
}
