#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut a:[usize;n],
        b:[Usize1;q],
    }
    a.sort();
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] + a[i]);
    }
    for b in b {
        let c = a.upper_bound(&b);
        if c == n {
            println!("-1");
            continue;
        }
        println!("{}", 1 + s[c] + (n - c) * b);
    }
}
