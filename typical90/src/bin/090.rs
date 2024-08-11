// https://atcoder.jp/contests/typical90/submissions/52251436
// 自力じゃ無理だったので上記をほぼ写経してます

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    let dp1 = f(n, k);
    let result = kitamasa(n, k, &dp1);
    println!("{}", result.val);
}

fn f(n: usize, k: usize) -> Vec<ModInt> {
    let mut dp = vec![vec![]; k + 1];
    dp[k] = vec![ModInt::one(); 3];
    for i in (1..k).rev() {
        let k_max = n.min(k / i);
        let j_max = dp[i + 1].len();
        let mut c = vec![ModInt::zero(); j_max];
        c[0] = ModInt::one();
        for j in 1..j_max {
            c[j] = ModInt::zero() - dp[i + 1][j];
        }
        dp[i] = polynomial_inverse(&c, k_max + 2);
    }

    dp[1].clone()
}

fn kitamasa(n: usize, k: usize, dp1: &[ModInt]) -> ModInt {
    let k = k.min(n);
    let mut v = vec![n + k + 1];
    while let Some(&x) = v.last() {
        if x < k + 1 {
            break;
        }
        v.push(x / 2);
    }
    v.reverse();
    let ref v = v;

    let mut cl = vec![ModInt::zero(); k + 2];
    cl[0] = ModInt::one();
    for i in 1..(k + 2) {
        cl[i] = ModInt::zero() - dp1[i];
    }

    let gl = polynomial_inverse(&cl, k + 2);
    cl.reverse();
    let ref cl = cl;

    let mut poly = vec![ModInt::zero(); k + 1];
    poly[v[0]] = ModInt::one(); // v[0] < k + 1
    for &x in &v[1..] {
        poly = ntt::convolve(&poly, &poly);
        if x % 2 == 1 {
            poly.insert(0, ModInt::zero());
        } else {
            poly.push(ModInt::zero());
        }

        let mut p = poly[(k + 1)..].to_vec();
        poly.truncate(k + 1);
        assert_eq!(p.len(), k + 1);
        p.reverse();

        p = ntt::convolve(&p, &gl);
        p.truncate(k + 1);
        p.reverse();

        p = ntt::convolve(&p, &cl);
        p.truncate(k + 1);

        for i in 0..(k + 1) {
            poly[i] -= p[i];
        }
    }

    poly[k]
}

// たこうしきのわりざん
fn polynomial_inverse(c: &[ModInt], l: usize) -> Vec<ModInt> {
    let n = c.len();
    let mut v = vec![ModInt::one(), ModInt::zero()];
    let k_max = (l * 2 - 1).ilog2();
    for k in 0..k_max {
        let k0 = 1 << k;
        v.resize(2 * k0, ModInt::zero());
        let p = ntt::convolve(&v, &c[..(n.min(2 * k0))]);
        let mut q = vec![ModInt::zero(); 2 * k0];
        q[0] = ModInt::one();
        for j in k0..(2 * k0) {
            q[j] = ModInt::zero() - p[j];
        }
        v = ntt::convolve(&v, &q);
    }

    v.truncate(l);
    v
}

const MOD: usize = 998244353;

#[derive(Clone, Copy)]
pub struct ModInt {
    val: usize,
}

impl ModInt {
    pub const fn zero() -> Self {
        Self { val: 0 }
    }

    pub const fn one() -> Self {
        Self { val: 1 }
    }

    pub fn new(i: usize) -> Self {
        ModInt { val: i % MOD }
    }

    pub fn inv(&self) -> Self {
        let mut a = self.val as i64;
        let mut b = MOD as i64;
        let modulo = MOD as i64;
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
        ModInt { val: u as usize }
    }

    pub fn pow(&self, mut n: usize) -> Self {
        let mut x = self.val;
        // let mut ret = if x == 0 { 0 } else { 1 };
        let mut ret = 1;
        while n > 0 {
            if n & 1 == 1 {
                ret = ret * x % MOD;
            }
            x = x * x % MOD;
            n >>= 1;
        }
        ModInt { val: ret }
    }
}

impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, other: Self) -> Self {
        ModInt::new(self.val + other.val)
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, other: Self) -> Self {
        ModInt::new(MOD + self.val - other.val)
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, other: Self) -> Self {
        ModInt::new(self.val * other.val)
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl std::fmt::Debug for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)?;
        Ok(())
    }
}

mod ntt {
    use super::ModInt;

    pub fn convolve(a: &Vec<ModInt>, b: &[ModInt]) -> Vec<ModInt> {
        let s = a.len() + b.len() - 1;
        let t = s.next_power_of_two();

        let root = {
            let z_t_n = ModInt::new(3).pow(119).pow((1 << 23) / t);
            (0..t).map(|i| z_t_n.pow(i)).collect::<Vec<_>>()
        };

        let root_inv = {
            let mut root_inv = root.clone();
            root_inv[1..].reverse();
            root_inv
        };

        let mut a = a.clone();
        let mut b = b.iter().map(|i| ModInt::new(i.val)).collect::<Vec<_>>();
        a.resize(t, ModInt::new(0));
        b.resize(t, ModInt::new(0));
        let a_inv = ntt(&a, &root);
        let b_inv = ntt(&b, &root);

        let c_inv = a_inv
            .into_iter()
            .zip(b_inv.into_iter())
            .map(|(a, b)| a * b)
            .collect();
        let c = ntt(&c_inv, &root_inv);

        let t_inv = ModInt::new(t).inv();
        c.into_iter().take(s).map(|x| x * t_inv).collect()
    }

    pub fn ntt(a: &Vec<ModInt>, root: &[ModInt]) -> Vec<ModInt> {
        let n = a.len();
        let d = n.trailing_zeros();

        let mask = n - 1;
        let mut res = a.clone();

        for i in (0..d).map(|i| (n - 1) >> i + 1) {
            res = (0..n)
                .map(|j| {
                    let l = i & j;
                    let u = j ^ l;
                    let s = u << 1 & mask;
                    res[s | l] + root[u] * res[s | i + 1 | l]
                })
                .collect();
        }

        res
    }
}
