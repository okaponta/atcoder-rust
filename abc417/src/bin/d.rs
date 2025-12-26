#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        n:usize,
        pab:[(usize,usize,usize);n],
        q:usize,
        x:[usize;q],
    }
    // i個目のプレゼントを受け取る前のテンションがjだったときの最終的なテンション
    let mut dp = vec![vec![0; 2001]; n + 1];
    for i in 0..2001 {
        dp[n][i] = i;
    }
    for i in (0..n).rev() {
        for j in 0..2001 {
            if j <= pab[i].0 {
                dp[i][j] = dp[i + 1][j + pab[i].1];
            } else {
                let k = if pab[i].2 < j { j - pab[i].2 } else { 0 };
                dp[i][j] = dp[i + 1][k];
            }
        }
    }
    let mut s = vec![1000];
    for i in 0..n {
        s.push(s[i] + pab[i].2);
    }
    for mut x in x {
        let p = s.lower_bound(&x);
        if n <= p {
            println!("{}", x + 1000 - s[n]);
        } else {
            x -= s[p] - 1000;
            println!("{}", dp[p][x]);
        }
    }
}
