use std::u8;

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
        q:usize,
    }
    let mut loc = vec![0; n];
    let mut num = vec![0; n];
    let mut ans = 0;
    for i in 0..n {
        loc[i] = i;
        num[i] = 1;
    }
    for _ in 0..q {
        input! {
            qi:u8
        }
        if qi == 1 {
            input! {
                p:Usize1,
                h:Usize1,
            }
            num[loc[p]] -= 1;
            if num[loc[p]] == 1 {
                ans -= 1;
            }
            loc[p] = h;
            num[h] += 1;
            if num[h] == 2 {
                ans += 1;
            }
        } else {
            println!("{}", ans);
        }
    }
}
