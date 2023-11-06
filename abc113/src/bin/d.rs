use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h:usize,
        w:usize,
        k:Usize1,
    }
    let mut lines = vec![vec![]];
    for i in 0..w - 1 {
        let tmp = lines.clone();
        for mut v in tmp {
            if v.len() == 0 || v[v.len() - 1] + 1 != i {
                v.push(i);
                lines.push(v);
            }
        }
    }
    let mut swap = vec![];
    for v in lines {
        let mut fw = HashSet::new();
        let mut bw = HashSet::new();
        for e in v {
            fw.insert(e);
            bw.insert(e + 1);
        }
        swap.push((fw, bw));
    }
    let mut dp = vec![ModInt::zero(); w];
    dp[0] = ModInt::one();
    for _ in 0..h {
        let mut next = vec![ModInt::zero(); w];
        for (fw, bw) in &swap {
            for i in 0..w {
                if fw.contains(&i) {
                    next[i + 1] += dp[i];
                } else if bw.contains(&i) {
                    next[i - 1] += dp[i];
                } else {
                    next[i] += dp[i];
                }
            }
        }
        dp = next;
    }
    println!("{}", dp[k].val);
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
