use proconio::{input, marker::Chars};

const MOD: usize = 998_244_353;

fn binom_init(n: usize, modulo: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![0; n + 1]; n + 1];
    res[0][0] = 1;
    for i in 1..=n {
        res[i][0] = 1;
        res[i][i] = 1;
        for j in 1..i {
            res[i][j] = (res[i - 1][j - 1] + res[i - 1][j]) % modulo;
        }
    }
    res
}

fn main() {
    input! {
       mut s:Chars,
    }
    let n = s.len();
    s.sort();
    let mut map = vec![0; 26];
    for e in s {
        map[(e as u8 - b'a') as usize] += 1;
    }
    let binom = binom_init(n, MOD);

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..26 {
        let mut next = vec![0; n + 1];
        for j in 0..=n {
            for k in 0..=j.min(map[i]) {
                next[j] += dp[j - k] * binom[j][k];
                next[j] %= MOD;
            }
        }
        dp = next;
    }
    dp.remove(0);
    println!("{}", dp.iter().fold(0, |acc, x| (acc + x) % MOD));
}
