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
        let mut target = vec![];
        for i in 0..m {
            if l <= i && i <= r {
                continue;
            }
            target.push(x[i]);
        }
        target.sort();
        let mut ans = 0;
        for (w, v) in wv.clone() {
            let i = target.lower_bound(&w);
            if i == target.len() {
                // 対象外
                continue;
            }
            ans += v;
            target.remove(i);
        }
        println!("{}", ans);
    }
}
