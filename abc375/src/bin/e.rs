use itertools::*;
use proconio::*;

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n],
    }
    let s = ab.iter().map(|(_, b)| b).sum::<usize>();
    if s % 3 != 0 {
        println!("-1");
        return;
    }
    let s = s / 3;
    let mut dp = vec![vec![300; s + 1]; s + 1];
    dp[0][0] = 0;
    let f = |a: usize, i: usize| if a == i { 0 } else { 1 };
    for i in 0..n {
        let mut next = vec![vec![300; s + 1]; s + 1];
        for (j, k) in iproduct!(0..=s, 0..=s) {
            if j + ab[i].1 <= s {
                next[j + ab[i].1][k] = next[j + ab[i].1][k].min(dp[j][k] + f(ab[i].0, 1));
            }
            if k + ab[i].1 <= s {
                next[j][k + ab[i].1] = next[j][k + ab[i].1].min(dp[j][k] + f(ab[i].0, 2));
            }
            next[j][k] = next[j][k].min(dp[j][k] + f(ab[i].0, 3));
        }
        dp = next;
    }
    if dp[s][s] == 300 {
        println!("-1");
        return;
    }
    println!("{}", dp[s][s]);
}
