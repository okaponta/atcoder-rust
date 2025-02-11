#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:[usize;n],
    }
    let mut a = vec![0; 1000005];
    for &s in &s {
        a[s] += 1;
    }
    let c = ntt::convolve(&a, &a);
    let mut ans = 0;
    for &s in &s {
        ans += (c[s * 2].val - 1) / 2;
    }
    println!("{}", ans);
}

// const MOD: usize = 1_000_000_007;
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

    pub fn convolve(a: &Vec<usize>, b: &Vec<usize>) -> Vec<ModInt> {
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

        let mut a = a.iter().copied().map(ModInt::new).collect::<Vec<_>>();
        let mut b = b.iter().copied().map(ModInt::new).collect::<Vec<_>>();
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

    fn ntt(a: &Vec<ModInt>, root: &Vec<ModInt>) -> Vec<ModInt> {
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
