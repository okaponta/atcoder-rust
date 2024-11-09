#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        x:[Usize1;m],
        a:[usize;m],
    }
    let mut xa = vec![];
    let mut s = 0;
    for i in 0..m {
        xa.push((x[i], a[i]));
        s += a[i];
    }
    xa.sort();
    if s != n || xa[0].0 != 0 {
        println!("-1");
        return;
    }
    let mut prev = 0;
    let mut rem = 0;
    let mut ans = n * (n - 1) / 2;
    for (x, a) in xa {
        if rem < x - prev {
            println!("-1");
            return;
        }
        rem -= x - prev;
        prev = x;
        rem += a;
        ans -= x * a;
    }
    println!("{}", ans);
}
