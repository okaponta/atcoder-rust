use std::collections::VecDeque;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let mut t = 0;
    let mut qu = VecDeque::new();
    for _ in 0..q {
        input! {qi:u8}
        if qi == 1 {
            qu.push_back(t);
        } else if qi == 2 {
            input! {ti: usize}
            t += ti;
        } else {
            input! {hi: usize}
            let mut ans = 0;
            while let Some(ti) = qu.pop_front() {
                if hi <= t - ti {
                    ans += 1;
                } else {
                    qu.push_front(ti);
                    break;
                }
            }
            println!("{ans}");
        }
    }
}
