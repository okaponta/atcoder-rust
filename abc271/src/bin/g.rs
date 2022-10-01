use proconio::{input, marker::Chars};

const MOD: i64 = 998244353;

fn main() {
    input! {
        n:usize,
        x:i64,
        y:i64,
        c:Chars
    }
    let inv100 = modinv(100, MOD);
    let x = x * inv100 % MOD;
    let y = y * inv100 % MOD;
    let mut prob = vec![0; 24];
    for i in 0..24 {
        prob[i] = if c[i] == 'T' { x } else { y };
    }
    let mut all_ng = 1;
    for i in 0..24 {
        all_ng *= 1 - prob[i];
        all_ng %= MOD;
    }
    let all_ng_sum = modinv(MOD + 1 - all_ng, MOD);

    let mut mat = vec![vec![0; 24]; 24];
    for i in 0..24 {
        for j in 0..24 {
            let mut c = 1;
            let mut k = i;
            while k != j {
                c *= 1 - prob[k];
                c %= MOD;
                k = (k + 1) % 24;
            }
            mat[(j + 1) % 24][i] += prob[j] * (c * all_ng_sum % MOD) % MOD;
            mat[(j + 1) % 24][i] %= MOD;
        }
    }

    let tr_rep = pow(mat.clone(), n - 1, MOD, 24);
    let prev = tr_rep.iter().map(|v| v[0]).collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..24 {
        for j in 0..24 {
            if c[j] == 'A' {
                ans += mat[(j + 1) % 24][i] * prev[i] % MOD;
                ans %= MOD;
            }
        }
    }

    println!("{}", ans.rem_euclid(MOD));
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

fn pow(mut a: Vec<Vec<i64>>, mut n: usize, modulo: i64, size: usize) -> Vec<Vec<i64>> {
    let mut b = vec![vec![0; size]; size];
    for i in 0..size {
        b[i][i] = 1;
    }
    while 0 < n {
        if n & 1 == 1 {
            b = multiply(&b, &a, MOD, size);
            rem(&mut b, modulo, size);
        }
        a = multiply(&a, &a, MOD, size);
        rem(&mut a, modulo, size);
        n >>= 1;
    }
    b
}

fn rem(a: &mut Vec<Vec<i64>>, modulo: i64, n: usize) {
    for i in 0..n {
        for j in 0..n {
            a[i][j] %= modulo;
        }
    }
}

// 行列式の掛け算
fn multiply(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>, modulo: i64, n: usize) -> Vec<Vec<i64>> {
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                res[i][j] += a[i][k] * b[k][j];
                res[i][j] %= modulo;
            }
        }
    }
    res
}
