#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        l:usize,
        r:usize,
        xy:[(usize,usize);n],
    }
    println!(
        "{}",
        xy.into_iter().filter(|&(x, y)| x <= l && r <= y).count()
    );
}
