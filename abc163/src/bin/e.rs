use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut ord = vec![];
    for i in 0..n {
        ord.push((a[i], i as i64));
    }
    ord.sort();
    let mut dp = vec![0; n + 1];
    for i in 0..n {
        let mut next = vec![0; n + 1];
        let (a, idx) = ord[n - i - 1];
        for j in 0..=i {
            next[j] = next[j].max(dp[j] + a * (idx - (n + j - i - 1) as i64).abs());
            next[j + 1] = dp[j] + a * (idx - j as i64).abs();
        }
        dp = next;
    }
    println!("{}", dp.iter().max().unwrap());
}
