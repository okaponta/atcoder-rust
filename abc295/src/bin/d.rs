use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut dp = vec![0; 1 << 10];
    let mut ans = 0usize;
    for c in s {
        let c = c.to_digit(10).unwrap() as usize;
        let mut next = vec![0; 1 << 10];
        for i in 0..1 << 10 {
            next[i ^ 1 << c] += dp[i];
        }
        next[1 << c] += 1;
        ans += next[0];
        dp = next;
    }
    println!("{}", ans);
}
