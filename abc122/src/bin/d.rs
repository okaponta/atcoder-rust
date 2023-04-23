use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut dp = vec![vec![vec![ModInt::zero(); 4]; 4]; 4];
    dp[3][3][3] = ModInt::one();
    for _ in 0..n {
        let mut next = vec![vec![vec![ModInt::zero(); 4]; 4]; 4];
        for (i, j, k) in iproduct!(0..4, 0..4, 0..4) {
            if (j, k) == (0, 1) || (j, k) == (1, 0) || (i, j) == (0, 1) || (i, k) == (0, 1) {
                next[j][k][1] += dp[i][j][k];
            } else if (j, k) == (0, 2) {
                next[j][k][2] += dp[i][j][k];
            } else {
                next[j][k][1] += dp[i][j][k];
                next[j][k][2] += dp[i][j][k];
            }
            next[j][k][0] += dp[i][j][k];
            next[j][k][3] += dp[i][j][k];
        }
        dp = next;
    }
    let mut ans = ModInt::zero();
    for (i, j, k) in iproduct!(0..4, 0..4, 0..4) {
        ans += dp[i][j][k];
    }
    println!("{}", ans.val);
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
