#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        lr:[(Usize1, usize);m],
    }
    let mut imos = vec![0; n + 1];
    for (l, r) in lr {
        imos[l] += 1;
        imos[r] -= 1;
    }
    let mut ans = imos[0];
    for i in 1..n {
        imos[i] += imos[i - 1];
        ans = ans.min(imos[i]);
    }
    println!("{}", ans);
}
