#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        q:usize,
        ht:[(char,Usize1);q],
    }
    let mut l = 0;
    let mut r = 1;
    let mut ans = 0;
    for (h, t) in ht {
        if h == 'L' {
            if l.min(t) < r && r < l.max(t) {
                ans += n + l.min(t) - l.max(t);
            } else {
                ans += l.max(t) - l.min(t);
            }
            l = t;
        } else {
            if r.min(t) < l && l < r.max(t) {
                ans += n + r.min(t) - r.max(t);
            } else {
                ans += r.max(t) - r.min(t);
            }
            r = t;
        }
    }
    println!("{}", ans);
}
