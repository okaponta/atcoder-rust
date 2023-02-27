use itertools::iproduct;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        q:usize,
        lr:[(usize,usize);m],
        pq:[(usize,usize);q],
    }
    let mut s = vec![vec![0; n + 1]; n + 1];
    for (l, r) in lr {
        s[l][r] += 1;
    }
    for (i, j) in iproduct!(0..n, 0..n) {
        s[i][j + 1] += s[i][j];
    }
    for (p, q) in pq {
        println!("{}", (p..=q).fold(0, |a, i| a + s[i][q]));
    }
}
