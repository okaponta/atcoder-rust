use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        mut n:usize,
    }
    let mut div = 9;
    let mut ans = 0;
    while n > div {
        let tmp = ((1 + div) % MOD) * (div % MOD) / 2;
        ans = (ans + tmp) % MOD;
        n = n - div;
        div = div * 10;
    }
    let tmp = ((1 + n) % MOD) * (n % MOD) / 2;
    ans = (ans + tmp) % MOD;
    println!("{}", ans);
}
