#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;n],
    }
    let mut ans = 0;
    loop {
        if (1..=m).any(|i| !a.contains(&i)) {
            println!("{ans}");
            return;
        }
        a.pop();
        ans += 1;
    }
}
