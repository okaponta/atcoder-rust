#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        p:[usize;n],
        q:[usize;n],
    }
    let mut ans = 0;
    for v in (1..=n).permutations(n) {
        if p < v && v < q {
            ans += 1;
        }
    }
    println!("{}", ans);
}
