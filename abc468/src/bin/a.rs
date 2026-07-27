#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    println!(
        "{}",
        (1..n - 1)
            .filter(|i| a[i - 1] < a[*i] && a[*i] > a[i + 1])
            .count()
    );
}
