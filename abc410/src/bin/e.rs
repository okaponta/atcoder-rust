use proconio::*;

fn main() {
    input! {
        n:usize,
        h:usize,
        m:usize,
        ab:[(usize,usize);n],
    }
    let mut dp = vec![1 << 60; m + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut next = vec![1 << 60; m + 1];
        for j in 0..m {
            if dp[j] + ab[i].0 <= h {
                next[j] = next[j].min(dp[j] + ab[i].0);
            }
            if j + ab[i].1 <= m {
                next[j + ab[i].1] = next[j + ab[i].1].min(dp[j]);
            }
        }
        if next.iter().all(|&i| i == 1 << 60) {
            println!("{}", i);
            return;
        }
        dp = next;
    }
    println!("{}", n);
}
