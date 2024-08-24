#[allow(unused)]
use itertools::*;
use num_integer::Roots;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = 0;
    for i in 0..n {
        let mut tmp = 0;
        for (_, j) in factorize(a[i]) {
            tmp += j;
        }
        ans ^= tmp;
    }
    println!("{}", if ans == 0 { "Bruno" } else { "Anna" });
}

fn factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for i in 2..=n.sqrt() {
        if n % i != 0 {
            continue;
        }
        let mut ex = 0;
        while n % i == 0 {
            ex += 1;
            n /= i;
        }
        res.push((i, ex));
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}
