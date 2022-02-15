use proconio::{input, marker::Usize1};

const MOD: usize = 998244353;

fn main() {
    input! {
        h:usize,w:usize,k:usize,
        xy1:(Usize1,Usize1),xy2:(Usize1,Usize1),
    }
    // 始点、その上下、その左右、その他
    let mut dp = vec![1, 0, 0, 0];
    for _ in 0..k {
        let mut next = vec![0; 4];
        next[0] = ((h - 1) * dp[1] + (w - 1) * dp[2]) % MOD;
        next[1] = (dp[0] + (h - 2) * dp[1] + (w - 1) * dp[3]) % MOD;
        next[2] = (dp[0] + (w - 2) * dp[2] + (h - 1) * dp[3]) % MOD;
        next[3] = (dp[1] + dp[2] + (h + w - 4) * dp[3]) % MOD;
        dp = next;
    }
    if xy1.0 == xy2.0 {
        println!("{}", if xy1.1 == xy2.1 { dp[0] } else { dp[2] });
    } else {
        println!("{}", if xy1.1 == xy2.1 { dp[1] } else { dp[3] });
    }
}
