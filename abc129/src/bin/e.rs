use proconio::{input, marker::Chars};

fn main() {
    input! {
        l:Chars,
    }
    const MOD: usize = 1_000_000_007;
    let n = l.len();
    let mut mag = 1;
    let mut tri = vec![1];
    for i in 0..n {
        tri.push((tri[i] * 3) % MOD);
    }
    let mut ans = 0;
    for i in 0..n {
        if l[i] == '1' {
            ans = (ans + mag * tri[n - 1 - i]) % MOD;
            mag = (mag * 2) % MOD;
        }
    }
    println!("{}", (ans + mag) % MOD);
}
