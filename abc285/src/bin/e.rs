use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] + a[i / 2]);
    }
    let mut dp = vec![0; n + 1];
    for i in 0..=n {
        let mut next = vec![0; n + 1];
        for j in 0..i {
            next[j + 1] = dp[j];
            next[0] = next[0].max(dp[j] + s[j]);
        }
        dp = next;
    }
    println!("{}", dp[0]);
}
