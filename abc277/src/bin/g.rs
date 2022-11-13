use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        uv:[(Usize1,Usize1);m],
        c:[u8;n],
    }
    let mut edges = vec![vec![]; n];
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut ans = 0;
    // dp[i][j] == iの頂点に0をj個決めた(3個以上は不要)
    let mut dp = vec![vec![0; 3]; n];
    dp[0][0] = 1;
    for _ in 0..k {
        let mut next = vec![vec![0; 3]; n];
        for i in 0..n {
            let modinv = modinv(edges[i].len() as i64, 998244353) as usize;
            for j in 0..3 {
                for &nx in &edges[i] {
                    next[nx][j] += dp[i][j] * modinv;
                    next[nx][j] %= 998244353;
                    if c[nx] == 0 && j < 2 {
                        next[nx][j + 1] += dp[i][j] * modinv;
                        next[nx][j + 1] %= 998244353;
                    }
                }
            }
        }
        for i in 0..n {
            if c[i] == 1 {
                ans += next[i][1] + next[i][2] * 2;
                ans %= 998244353;
            }
        }
        dp = next;
    }
    println!("{}", ans);
}

fn modinv(mut a: i64, modulo: i64) -> i64 {
    let mut b = modulo;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= modulo;
    if u < 0 {
        u += modulo;
    }
    u
}
