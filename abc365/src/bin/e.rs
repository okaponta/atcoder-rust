use std::mem::swap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = 0i64;
    for i in 0..30 {
        let mut cnt = 0;
        let mut zero = 0;
        let mut one = 0;
        for j in 0..n {
            if a[j] >> i & 1 == 0 {
                cnt += one;
                zero += 1;
            } else {
                cnt += zero;
                swap(&mut zero, &mut one);
                one += 1;
            }
        }
        ans += (1 << i) * cnt;
    }
    println!("{}", ans);
}
