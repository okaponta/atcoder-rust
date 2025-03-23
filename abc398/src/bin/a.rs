#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
    }
    let mut ans = (0..n).into_iter().map(|_| '-').collect::<Vec<_>>();
    ans[n / 2] = '=';
    if n % 2 == 0 {
        ans[(n / 2) - 1] = '=';
    }
    println!("{}", ans.iter().join(""));
}
