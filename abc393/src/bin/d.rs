#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut m = 0;
    let mut tmp = 0;
    let mut imos = vec![0; n + 1];
    for i in 0..n {
        if s[i] == '1' {
            m += 1;
            tmp += i;
            imos[i + 1 - m] += 2;
        }
    }
    tmp -= m * (m - 1) / 2;
    let mut tmp = tmp as i64;
    let mut diff = -(m as i64);
    let mut ans = tmp;
    for i in 0..=n - m {
        diff += imos[i];
        tmp += diff;
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}
