use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        c:[Chars;h],
    }
    let mask = (1 << (w + 1)) - 1;
    let mut dp = HashMap::new();
    dp.insert(0, ModInt::one());
    for i in 0..h {
        for j in 0..w {
            let mut next = HashMap::new();
            for (k, v) in dp {
                if c[i][j] == '.' && can_put_king(h, w, i, j, k) {
                    *next.entry(((k << 1) + 1) & mask).or_insert(ModInt::zero()) += v;
                }
                *next.entry((k << 1) & mask).or_insert(ModInt::zero()) += v;
            }
            dp = next;
        }
    }
    println!(
        "{}",
        dp.into_iter().fold(ModInt::zero(), |s, v| s + v.1).val
    );
}

fn can_put_king(h: usize, w: usize, i: usize, j: usize, s: usize) -> bool {
    for (di, dj, b) in vec![(0, !0, 0), (!0, !0, w), (!0, 0, w - 1), (!0, 1, w - 2)] {
        let ni = i.wrapping_add(di);
        let nj = j.wrapping_add(dj);
        if h <= ni || w <= nj {
            continue;
        }
        if s >> b & 1 == 1 {
            return false;
        }
    }
    true
}

const MOD: usize = 1_000_000_007;

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
