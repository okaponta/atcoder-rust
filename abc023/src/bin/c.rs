use proconio::{marker::*, *};
use std::collections::HashMap;

fn main() {
    input! {
        r:usize,
        c:usize,
        k:usize,
        n:usize,
        rc:[(Usize1,Usize1);n],
    }
    let mut rcnt = vec![0; r];
    let mut ccnt = vec![0; c];
    for &(i, j) in &rc {
        rcnt[i] += 1;
        ccnt[j] += 1;
    }
    let mut cmap = HashMap::new();
    for i in 0..c {
        *cmap.entry(ccnt[i]).or_insert(0usize) += 1;
    }
    let mut ans = 0;
    for i in 0..r {
        if k < rcnt[i] {
            continue;
        }
        ans += cmap.get(&(k - rcnt[i])).unwrap_or(&0);
    }
    for &(i, j) in &rc {
        if rcnt[i] + ccnt[j] == k {
            ans -= 1;
        }
        if rcnt[i] + ccnt[j] == k + 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
