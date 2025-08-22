#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        l:usize,
        d:[usize;n-1],
    }
    if l % 3 != 0 {
        println!("0");
        return;
    }
    let mut count = vec![0usize; l];
    let mut tmp = 0;
    count[0] = 1;
    for &d in &d {
        tmp = (tmp + d) % l;
        count[tmp] += 1;
    }
    let mut ans = 0;
    for i in 0..l / 3 {
        ans += count[i] * count[i + l / 3] * count[i + (2 * l / 3)];
    }
    println!("{}", ans);
}
