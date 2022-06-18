use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut zero = vec![0; 60];
    let mut one = vec![0; 60];
    let mut ans = vec![0; 60];
    for ai in a {
        let mut dig = vec![0; 60];
        for i in 0..60 {
            if ai >> i & 1 == 1 {
                dig[i] += 1;
            }
        }
        for i in 0..60 {
            if dig[i] == 1 {
                ans[i] += zero[i];
            } else {
                ans[i] += one[i];
            }
            ans[i] %= MOD;
        }
        for i in 0..60 {
            if dig[i] == 1 {
                one[i] += 1;
            } else {
                zero[i] += 1;
            }
        }
    }
    let mut a = 0;
    let mut pow2 = 1;
    for i in 0..60 {
        a += ans[i] * pow2;
        a %= MOD;
        pow2 = pow2 * 2 % MOD;
    }
    println!("{}", a);
}
