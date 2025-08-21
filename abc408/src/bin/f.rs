#[rustfmt::skip]
use {itertools::*,proconio::{marker::*, *}, ac_library::*};
fn main() {
    input! {
        n:usize,d:usize,r:usize,
        h:[Usize1;n]
    }
    let p = (0..n)
        .into_iter()
        .sorted_by(|&a, &b| h[a].cmp(&h[b]))
        .collect_vec();
    let mut s = Segtree::<Max<usize>>::new(n);
    let mut dp = vec![0; n];
    let mut ans = 0;
    for i in 0..n {
        if d <= i {
            s.set(p[i - d], dp[i - d]);
        }
        let l = p[i].saturating_sub(r);
        dp[i] = dp[i].max(s.prod(l..n.min(p[i] + r + 1)) + 1);
        ans = ans.max(dp[i] - 1);
    }
    println!("{ans}");
}
