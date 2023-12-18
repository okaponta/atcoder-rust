use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[usize;n],
    }
    let mut dp = vec![0; 10];
    for s in s {
        let mut next = dp.clone();
        for i in 0..10 {
            if i != 0 && dp[i] == 0 {
                continue;
            }
            next[(i + s) % 10] = next[(i + s) % 10].max(dp[i] + s);
        }
        dp = next;
    }
    println!("{}", (1..10).into_iter().map(|i| dp[i]).max().unwrap());
}
