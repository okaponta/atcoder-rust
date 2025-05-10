#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let ans = (0..n).step_by(2).map(|i| a[i]).sum::<usize>();
    println!("{}", ans);
}
