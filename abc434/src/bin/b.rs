#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,usize);n],
    }
    let mut count = vec![0; m];
    let mut sum = vec![0; m];
    for (a, b) in ab {
        count[a] += 1;
        sum[a] += b;
    }
    for i in 0..m {
        println!("{}", sum[i] as f64 / count[i] as f64);
    }
}
