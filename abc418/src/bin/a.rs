#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    if n < 3 {
        println!("No");
        return;
    }
    if s[n - 3] == 't' && s[n - 2] == 'e' && s[n - 1] == 'a' {
        println!("Yes");
    } else {
        println!("No");
    }
}
