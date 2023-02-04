use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        a:[i64;n],
        q:usize,
        lr:[(Usize1,usize);q],
    }
    let mut s = vec![vec![0; k]];
    for i in 0..n {
        let mut next = s[i].clone();
        next[i % k] += a[i];
        s.push(next);
    }
    for (l, r) in lr {
        let ok = (0..k).into_iter().map(|i| s[l][i] - s[r][i]).all_equal();
        println!("{}", if ok { "Yes" } else { "No" });
    }
}
