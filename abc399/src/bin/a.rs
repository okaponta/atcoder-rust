#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        s:Chars,
        t:Chars,
    }
    let ans = (0..n).into_iter().filter(|&i| s[i] != t[i]).count();
    println!("{}", ans);
}
