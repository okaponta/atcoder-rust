#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut ans = 1;
    let mut tmp = 1;
    for _i in 0..m {
        tmp *= n;
        ans += tmp;
        if 1_000_000_000 < ans {
            println!("inf");
            return;
        }
    }
    println!("{}", ans);
}
