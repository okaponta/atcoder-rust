#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        k:[usize;n],
    }
    let mut ans = 1 << 60;
    let sum = k.iter().sum::<usize>();
    for i in 0..1 << n {
        let mut tmp = 0;
        for j in 0..n {
            if i >> j & 1 == 1 {
                tmp += k[j];
            }
        }
        ans = ans.min(tmp.max(sum - tmp));
    }
    println!("{}", ans);
}
