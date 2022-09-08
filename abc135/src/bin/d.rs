use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        s:Chars,
    }
    let mut dp = vec![0; 13];
    dp[0] = 1;
    for c in s {
        let base = mul10(dp);
        let mut next = vec![0; 13];
        if c == '?' {
            let mut s = vec![0];
            for i in 0..26 {
                s.push((s[i] + base[i % 13]) % MOD);
            }
            for i in 0..13 {
                next[i] = (MOD + s[i + 14] - s[i + 4]) % MOD;
            }
        } else {
            let digit = c.to_digit(10).unwrap() as usize;
            for i in 0..13 {
                next[(i + digit) % 13] = base[i];
            }
        }
        dp = next;
    }
    println!("{}", dp[5]);
}

fn mul10(bef: Vec<usize>) -> Vec<usize> {
    let mut res = vec![0; 13];
    for i in 0..13 {
        res[i * 10 % 13] = bef[i];
    }
    res
}
