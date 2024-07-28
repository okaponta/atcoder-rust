use proconio::*;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
    }
    println!("{}", a * b % MOD * c % MOD);
}
