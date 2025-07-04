#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut s = a.iter().sum::<usize>();
    let mut ans = 0;
    for i in 0..n {
        s -= a[i];
        ans += a[i] * s;
    }
    println!("{}", ans);
}
