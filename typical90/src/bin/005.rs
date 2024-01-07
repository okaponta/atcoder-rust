use itertools::iproduct;
use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        mut n:usize,
        b:usize,
        k:usize,
        c:[usize;k],
    }
    // 10^1 -> 10^2 -> 10^4 -> 10^8をbで割った余りを保持する
    let mut dp = vec![0; b];
    c.into_iter().for_each(|i| dp[i % b] += 1);

    let mut ans = vec![0; b];
    ans[0] = 1;
    // dは10^1 -> 10^2 -> 10^4 -> 10^8...と遷移する
    let mut d = 10;
    while n > 0 {
        if n & 1 == 1 {
            ans = mul(&ans, &dp, d);
        }
        dp = mul(&dp, &dp, d);
        d = (d * d) % b;
        n >>= 1;
    }

    println!("{}", ans[0]);
}

// 前半i桁を割ったあまりをp
// 後半j桁を割ったあまりをq
// i+j桁を割ったあまりは(p*d+q)%b
// dp[i+j][(p*d+q)%b] += dp[i][p] * dp[j][q] (シンプルに掛け合わせ)
// 前半の余りがa、後半の余りがb、bの桁数がdのときに余りがどうなるかを返却する
fn mul(a: &Vec<usize>, b: &Vec<usize>, d: usize) -> Vec<usize> {
    let n = a.len();
    let mut res = vec![0; n];
    for (i, j) in iproduct!(0..n, 0..n) {
        let next = (i * d + j) % n;
        res[next] += a[i] * b[j];
        res[next] %= MOD;
    }
    res
}

// 貪欲解、計算量O(NBK)
// fn main() {
//     input! {
//         n:usize,
//         b:usize,
//         k:usize,
//         c:[usize;k],
//     }
//     // dp[i][j] i桁でbで割ったあまりがjの個数
//     let mut dp = vec![vec![0; b]; n + 1];
//     dp[0][0] = 1;
//     for i in 0..n {
//         for j in 0..b {
//             for k in 0..k {
//                 dp[i + 1][(j * 10 + c[k]) % b] += dp[i][j];
//                 dp[i + 1][(j * 10 + c[k]) % b] %= MOD;
//             }
//         }
//     }
//     println!("{}", dp[n][0]);
// }
