use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        csf:[(usize,usize,usize);n-1],
    }
    let mut dp = vec![0; n];
    for i in 0..n - 1 {
        for j in 0..=i {
            dp[j] = (dp[j] + (csf[i].2 - dp[j] % csf[i].2) % csf[i].2).max(csf[i].1) + csf[i].0;
        }
    }
    for ans in dp {
        println!("{}", ans);
    }
}
