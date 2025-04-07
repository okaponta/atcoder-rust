#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};
use num_integer::Roots;

fn main() {
    input! {
        mut n:usize,
    }
    n /= 2;
    let mut ans = n.sqrt();
    n /= 2;
    ans += n.sqrt();
    println!("{}", ans);
}
