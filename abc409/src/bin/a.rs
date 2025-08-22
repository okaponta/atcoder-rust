#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        t:Chars,
        a:Chars,
    }
    println!(
        "{}",
        if (0..n).any(|i| t[i] == 'o' && a[i] == 'o') {
            "Yes"
        } else {
            "No"
        }
    );
}
