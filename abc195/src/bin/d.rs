use std::cmp::Reverse;

use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
       n:usize,m:usize,q:usize,
       mut wv:[(usize,usize);n],
       x:[usize;m],
       lr:[(Usize1,Usize1);q],
    }
    wv.sort_by_key(|k| Reverse(k.1));
    for (l, r) in lr {
        let mut xc = x.clone();
        xc.drain(l..(r + 1));
        xc.sort();
        let mut ans = 0;
        for (w, v) in wv.clone() {
            let i = xc.lower_bound(&w);
            if i != xc.len() {
                ans += v;
                xc.remove(i);
            }
        }
        println!("{}", ans);
    }
}
