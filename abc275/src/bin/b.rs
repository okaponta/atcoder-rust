use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        d:usize,
        e:usize,
        f:usize,
    }
    let one = (a % MOD) * (b % MOD) % MOD * (c % MOD) % MOD;
    let two = (d % MOD) * (e % MOD) % MOD * (f % MOD) % MOD;
    println!("{}", (one + MOD - two) % MOD);
}
