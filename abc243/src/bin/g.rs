use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        t:usize,
        x:[usize;t],
    }
    x.iter().for_each(|xi| testcase(*xi));
}

fn testcase(x: usize) {
    let max = x.sqrt().sqrt();
    let mut dp = vec![0; max + 1];
    dp[1] = 1;
    for i in 1..=max {
        let mut count = 0;
        for j in 1..=i.sqrt() {
            count += dp[j];
        }
        dp[i] = count;
    }

    let sq = x.sqrt();
    let mut ans = 0;
    for i in 1..=max {
        ans += (sq - i * i + 1) * dp[i];
    }
    println!("{}", ans);
}
