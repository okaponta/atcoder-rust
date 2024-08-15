#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        x:usize,
        y:usize,
        a:[usize;n],
        b:[usize;n],
    }
    let mut ab = vec![];
    for i in 0..n {
        ab.push((a[i], b[i]));
    }
    let mut ans = 1 << 60;
    ab.sort();
    ab.reverse();
    ans = ans.min(f(x, y, &ab));
    ab.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
    ans = ans.min(f(x, y, &ab));
    println!("{}", ans);
}

fn f(x: usize, y: usize, ab: &Vec<(usize, usize)>) -> usize {
    let mut a = 0;
    let mut b = 0;
    for i in 0..ab.len() {
        if x < a + ab[i].0 || y < b + ab[i].1 {
            return i + 1;
        }
        a += ab[i].0;
        b += ab[i].1;
    }
    ab.len()
}
