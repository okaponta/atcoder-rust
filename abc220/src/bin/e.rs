use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n:usize,d:usize
    }
    let mut two_pow = vec![1; 2 * n + 1];
    for i in 1..2 * n {
        two_pow[i] = two_pow[i - 1] * 2 % MOD;
    }
    let mut ans = 0;
    for i in 0..n {
        let num = two_pow[i];
        let mut count = 0;
        if i + d < n {
            count += two_pow[d + 1];
        }
        if 2 * (n - i - 1) < d || d == 1 {
        } else if i + d < n {
            count += two_pow[d - 1] * (d as i64 - 1) % MOD;
        } else {
            count += two_pow[d - 1] * (2 * n as i64 - 2 * i as i64 - d as i64 - 1) % MOD;
        }
        ans = (ans + num * count % MOD) % MOD;
    }
    println!("{}", ans);
}
