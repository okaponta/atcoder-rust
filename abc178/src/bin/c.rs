use proconio::input;

const MOD: i64 = 1000000007;

fn pw(x: i64, n: i64) -> i64 {
    let mut ans = 1;
    for _i in 0..n {
        ans = (ans * x) % MOD;
    }
    return ans;
}

fn main() {
    input! {
       n:i64,
    }
    let ans = pw(10, n) - 2 * pw(9, n) + pw(8, n) + 2 * MOD;
    println!("{}", ans % MOD);
}
