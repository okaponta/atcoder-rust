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
        apbq:[(usize,usize,usize,usize);n],
    }
    let mut base = vec![];
    let mut knap = vec![];
    for (a, p, b, q) in apbq {
        if a * q < b * p {
            base.push((b, q));
        } else {
            base.push((a, p));
        }
        let mut tmp = vec![1 << 60; 20001];
        tmp[0] = 0;
        for i in 0..20000 {
            if i + a <= 20000 {
                tmp[i + a] = tmp[i + a].min(tmp[i] + p);
            }
            if i + b <= 20000 {
                tmp[i + b] = tmp[i + b].min(tmp[i] + q);
            }
        }
        let mut min = 1 << 60;
        for i in (0..=20000).rev() {
            min = min.min(tmp[i]);
            tmp[i] = tmp[i].min(min);
        }
        knap.push(tmp);
    }

    let mut lower = 0;
    let mut upper = x * 101;
    while 1 < upper - lower {
        let mid = (lower + upper) / 2;
        if f(n, mid, &base, &knap) <= x {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    println!("{}", lower);
}

// midの値を満たす予算
fn f(n: usize, w: usize, base: &Vec<(usize, usize)>, knap: &Vec<Vec<usize>>) -> usize {
    let mut res = 0;
    for i in 0..n {
        if 20000 < w {
            let pre = (w - 10000) / base[i].0;
            res += pre * base[i].1;
            res += knap[i][w - pre * base[i].0];
        } else {
            res += knap[i][w];
        }
    }
    res
}
