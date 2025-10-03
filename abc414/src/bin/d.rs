#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut x:[usize;n],
    }
    if n == 1 {
        println!("0");
        return;
    }
    x.sort();
    let ans = x[n - 1] - x[0];
    if m == 1 {
        println!("{}", ans);
        return;
    }
    let d = (1..n)
        .into_iter()
        .map(|i| x[i] - x[i - 1])
        .sorted()
        .rev()
        .collect_vec();
    let ans = x[n - 1] - x[0] - (0..m - 1).into_iter().map(|i| d[i]).sum::<usize>();
    println!("{}", ans);
}
