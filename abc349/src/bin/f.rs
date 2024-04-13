use num_integer::Roots;
use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
    }
    let fact = factorize(m);
    let mut ok = vec![];
    for (p, i) in fact {
        ok.push(p.pow(i));
    }
    let k = ok.len();
    let mut count = vec![0; 1 << k];
    let mut two = vec![1];
    for i in 0..n {
        two.push((two[two.len() - 1] * 2) % MOD);
        if a[i] == 1 {
            count[0] += 1;
            continue;
        }
        if m % a[i] != 0 {
            // 選んじゃダメ
            continue;
        }
        let mut tmp = 0;
        for j in 0..k {
            if a[i] % ok[j] == 0 {
                tmp += 1 << j;
            }
        }
        count[tmp] += 1;
    }
    let mut dp = vec![0; 1 << k];
    dp[0] = 1;
    for i in 0..1 << k {
        for j in (0..1 << k).rev() {
            dp[i | j] += dp[j] * (two[count[i]] - 1);
            dp[i | j] %= MOD;
        }
    }
    let mut ans = dp[(1 << k) - 1];
    if m == 1 {
        ans -= 1;
    }
    println!("{}", ans);
}

fn factorize(mut n: usize) -> Vec<(usize, u32)> {
    let mut res = vec![];
    for i in 2..=n.sqrt() {
        if n % i != 0 {
            continue;
        }
        let mut ex = 0;
        while n % i == 0 {
            ex += 1;
            n /= i;
        }
        res.push((i, ex));
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}
