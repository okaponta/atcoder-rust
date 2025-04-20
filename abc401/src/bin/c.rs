#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    let mut a = vec![1; n + 1];
    if n < k {
        println!("1");
        return;
    }
    a[k] = k;
    for i in k + 1..=n {
        a[i] = (1_000_000_000 + a[i - 1] * 2 - a[i - k - 1]) % 1_000_000_000;
    }
    println!("{}", a[n]);
}
