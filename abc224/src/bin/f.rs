use proconio::{input, marker::Chars};

const MOD: i64 = 998244353;

fn main() {
    input! {
       s:Chars,
    }
    let n = s.len();
    let mut pow2 = vec![1];
    for i in 0..n {
        pow2.push(pow2[i] * 2 % MOD);
    }
    let mut unfix = 0;
    let mut temp = 0;
    let mut sum = 0;
    for idx in 0..n {
        let i = s[idx].to_digit(10).unwrap() as i64;
        unfix = (unfix * 10 + i * pow2[idx]) % MOD;
        temp = (sum + unfix) % MOD;
        sum = (sum + temp) % MOD;
    }
    println!("{}", temp);
}
