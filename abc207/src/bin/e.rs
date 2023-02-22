use proconio::input;

fn main() {
    input! {
       n:usize,
       a:[usize;n],
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }
    let mut dp = vec![vec![ModInt::zero(); n + 1]; n + 1];
    let mut memo = vec![vec![ModInt::zero(); n + 1]; n + 1];
    dp[0][0] += ModInt::one();
    memo[1][0] += ModInt::one();
    for i in 0..n {
        for j in 1..=n {
            dp[i + 1][j] = memo[j][s[i + 1] % j];
        }
        for j in 2..=n {
            memo[j][s[i + 1] % j] += dp[i + 1][j - 1];
        }
    }
    let mut ans = ModInt::zero();
    for i in 0..=n {
        ans += dp[n][i];
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
        let mut ret = if x == 0 { 0 } else { 1 };
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
