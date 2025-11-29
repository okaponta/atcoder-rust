#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        case();
    }
}

fn case() {
    input! {
        n:usize,
        h:i64,
        tlu:[(i64,i64,i64);n],
    }
    let mut min = h;
    let mut max = h;
    let mut prev = 0;
    for (t, l, u) in tlu {
        let diff = t - prev;
        let nmin = min - diff;
        let nmax = max + diff;
        if nmax < l || u < nmin {
            println!("No");
            return;
        }
        min = nmin.max(l);
        max = nmax.min(u);
        prev = t;
    }
    println!("Yes");
}
