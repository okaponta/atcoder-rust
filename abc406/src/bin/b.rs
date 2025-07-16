#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        k:u32,
        a:[u128;n],
    }
    let mut tmp = 1;
    for a in a {
        tmp *= a;
        if 10u128.pow(k) <= tmp {
            tmp = 1;
        }
    }
    println!("{}", tmp);
}
