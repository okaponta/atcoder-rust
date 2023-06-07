use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut dp = (0..=n).into_iter().collect::<Vec<_>>();
    ope(n, 6, &mut dp);
    ope(n, 9, &mut dp);
    println!("{}", dp[n]);
}

fn ope(n: usize, i: usize, dp: &mut Vec<usize>) {
    let mut tmp = i;
    while tmp <= n {
        for i in 0..=n - tmp {
            dp[i + tmp] = dp[i + tmp].min(dp[i] + 1);
        }
        tmp *= i;
    }
}
