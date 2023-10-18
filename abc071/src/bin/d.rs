use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s1:Chars,
        s2:Chars,
    }
    if n == 1 {
        println!("3");
        return;
    }
    let mut dp = vec![ModInt::zero(); n];
    let mut i = 0;
    if s1[0] == s1[1] {
        dp[1] += ModInt::new(6);
        i = 1;
    } else {
        dp[0] += ModInt::new(3);
    }
    while i + 1 < n {
        if 0 < i && s1[i] != s2[i] {
            // よこ -> たて
            if s1[i + 1] == s2[i + 1] {
                dp[i + 1] = dp[i];
                i += 1;
            } else {
                // よこ -> よこ
                dp[i + 2] = dp[i] * ModInt::new(3);
                i += 2;
            }
        } else {
            // たて -> たて
            if s1[i + 1] == s2[i + 1] {
                dp[i + 1] = dp[i] * ModInt::new(2);
                i += 1;
            } else {
                // たて -> よこ
                dp[i + 2] = dp[i] * ModInt::new(2);
                i += 2;
            }
        }
    }
    println!("{}", dp[n - 1].val);
}

const MOD: usize = 1_000_000_007;
//const MOD: usize = 998244353;

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
