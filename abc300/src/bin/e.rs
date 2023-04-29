use proconio::input;

fn main() {
    input! {
        mut n:usize,
    }
    // 2,3,5
    let mut fact = vec![0; 3];
    while n % 2 == 0 || n % 3 == 0 || n % 5 == 0 {
        if n % 2 == 0 {
            fact[0] += 1;
            n /= 2;
        }
        if n % 3 == 0 {
            fact[1] += 1;
            n /= 3;
        }
        if n % 5 == 0 {
            fact[2] += 1;
            n /= 5;
        }
    }
    if n != 1 {
        println!("0");
        return;
    }
    let mut dp = vec![vec![vec![ModInt::zero(); fact[2] + 1]; fact[1] + 1]; fact[0] + 1];
    let fifth = ModInt::new(5).inv();
    dp[fact[0]][fact[1]][fact[2]] = ModInt::one();
    for i in (0..=fact[0]).rev() {
        for j in (0..=fact[1]).rev() {
            for k in (0..=fact[2]).rev() {
                if i != 0 {
                    dp[i - 1][j][k] = dp[i - 1][j][k] + dp[i][j][k] * fifth;
                }
                if j != 0 {
                    dp[i][j - 1][k] = dp[i][j - 1][k] + dp[i][j][k] * fifth;
                }
                if 1 < i {
                    dp[i - 2][j][k] = dp[i - 2][j][k] + dp[i][j][k] * fifth;
                }
                if k != 0 {
                    dp[i][j][k - 1] = dp[i][j][k - 1] + dp[i][j][k] * fifth;
                }
                if i != 0 && j != 0 {
                    dp[i - 1][j - 1][k] = dp[i - 1][j - 1][k] + dp[i][j][k] * fifth;
                }
            }
        }
    }
    println!("{:?}", dp[0][0][0].val);
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
