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
        lrs:[(Usize1,usize,i64);n],
    }
    let mut imos = vec![0; m + 1];
    let mut sum = 0;
    for (l, r, s) in lrs {
        sum += s;
        imos[l] += s;
        imos[r] -= s;
    }
    let rui = ruiseki(&imos);
    println!("{}", sum - rui.iter().min().unwrap());
}

fn ruiseki(a: &Vec<i64>) -> Vec<i64> {
    let mut res = vec![a[0]];
    for i in 1..a.len() {
        res.push(res[i - 1] + a[i]);
    }
    res.pop();
    res
}
