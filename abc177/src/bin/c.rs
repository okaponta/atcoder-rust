use proconio::input;

const MOD: i64 = 1000000007;

fn main() {
    input! {
       n:usize,a:[i64;n],
    }
    let s = a
        .iter()
        .scan(0, |state, &x| {
            *state = (*state + x) % MOD;
            Some(*state)
        })
        .collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..n {
        ans = (ans + a[i] * (s[n - 1] - s[i])) % MOD;
    }
    println!("{}", (ans + MOD) % MOD);
}
