#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        _n:usize,
        l:Usize1,
        r:usize,
        s:Chars
    }
    println!(
        "{}",
        if (l..r).all(|i| s[i] == 'o') {
            "Yes"
        } else {
            "No"
        }
    );
}
