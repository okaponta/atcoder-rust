use proconio::input;

fn main() {
    input! {
        x:usize,
    }
    if x % 9 != 0 {
        println!("0");
        return;
    }
    let mut dp = vec![0usize; x + 1];
    for i in 1..=x {
        if i < 10 {
            dp[i] += 1;
        }
        for j in i.saturating_sub(9)..i {
            dp[i] += dp[j];
        }
        dp[i] %= 1_000_000_007;
    }
    println!("{}", dp[x]);
}
