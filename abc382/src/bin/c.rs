#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        b:[usize;m],
    }
    let mut ans = vec![0; 200_020];
    let mut tmp = 200_010;
    for i in 0..n {
        while a[i] <= tmp {
            ans[tmp] = i + 1;
            tmp -= 1;
        }
    }
    for b in b {
        if ans[b] == 0 {
            println!("-1");
        } else {
            println!("{}", ans[b]);
        }
    }
}
