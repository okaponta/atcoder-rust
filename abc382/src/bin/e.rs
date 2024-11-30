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
        p:[usize;n],
    }
    let mut pp = vec![0.0; n + 1];
    pp[0] = 1.0;
    for &p in &p {
        let p = p as f64 / 100.0;
        for i in (0..n).rev() {
            let rare = pp[i] * p;
            pp[i + 1] += rare;
            pp[i] -= rare;
        }
    }
    let mut ans = vec![0.0; x + 1];
    for i in (0..x).rev() {
        let mut tmp = 1.0;
        let mut sum = 0.0;
        for j in 1..=n {
            if x <= i + j {
                break;
            }
            sum += pp[j] * ans[i + j];
        }
        tmp += sum;
        tmp /= 1.0 - pp[0];
        ans[i] = tmp;
    }
    println!("{}", ans[0]);
}
