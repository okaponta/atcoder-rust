use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    // average
    let mut lower = 0.0;
    let mut upper = 1e18;
    while upper - lower > 1e-5 {
        let med = (lower + upper) / 2.0;
        if is_ave_over(&a, med) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);

    // median
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_med_over(&a, med) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ave_over(a: &[i64], k: f64) -> bool {
    let a = a.iter().map(|&a| a as f64 - k).collect::<Vec<_>>();
    let mut dp = vec![a[0], 0.0];
    for i in 1..a.len() {
        let a = a[i];
        dp = vec![dp[0].max(dp[1]) + a, dp[0]];
    }
    dp[0].max(dp[1]) > 0.0
}

fn is_med_over(a: &[i64], k: i64) -> bool {
    // a>=kなら1、そうでないなら-1
    let a = a
        .iter()
        .map(|&a| if a >= k { 1 } else { -1 })
        .collect::<Vec<_>>();
    let mut dp = vec![a[0], 0];
    for i in 1..a.len() {
        let a = a[i];
        dp = vec![dp[0].max(dp[1]) + a, dp[0]];
    }
    dp[0].max(dp[1]) > 0
}
