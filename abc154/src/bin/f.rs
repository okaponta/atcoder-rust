use proconio::input;

fn main() {
    input! {
        r1:usize,
        c1:usize,
        r2:usize,
        c2:usize,
    }
    let c = Binom::init(r2 + c2);
    let ans = g(r2, c2, &c) - g(r1 - 1, c2, &c) - g(r2, c1 - 1, &c) + g(r1 - 1, c1 - 1, &c);
    println!("{}", ans.val);
}

fn g(r: usize, c: usize, comb: &Binom) -> ModInt {
    let mut dp = vec![ModInt::one()];
    let mut r1 = 0;
    let mut c1 = 0;
    let mut r2 = 0;
    let mut c2 = 0;
    for i in 0..r + c {
        let mut tmp = dp[i].clone();
        tmp *= ModInt::new(2);
        if r1 != r {
            r1 += 1;
        } else {
            tmp -= comb.get(r1 + c1, r1);
            c1 += 1;
        }

        if c2 != c {
            c2 += 1;
        } else {
            tmp -= comb.get(r2 + c2, r2);
            r2 += 1;
        }
        dp.push(tmp);
    }
    dp.into_iter().fold(ModInt::zero(), |s, i| s + i)
}

const MOD: usize = 1_000_000_007;

pub struct Binom {
    fact: Vec<ModInt>,
    fact_inv: Vec<ModInt>,
}

impl Binom {
    pub fn init(n: usize) -> Self {
        let mut fact = vec![ModInt::zero(); n + 1];
        let mut inv = vec![ModInt::zero(); n + 1];
        let mut fact_inv = vec![ModInt::zero(); n + 1];
        fact[0] = ModInt::one();
        fact[1] = ModInt::one();
        inv[1] = ModInt::one();
        fact_inv[0] = ModInt::one();
        fact_inv[1] = ModInt::one();
        for i in 2..(n + 1) {
            fact[i] = fact[i - 1] * ModInt::new(i);
            inv[i] = ModInt::zero() - inv[MOD % i] * ModInt::new(MOD / i);
            fact_inv[i] = fact_inv[i - 1] * inv[i];
        }
        Self { fact, fact_inv }
    }

    // nCk
    pub fn get(&self, n: usize, k: usize) -> ModInt {
        assert!(n >= k);
        self.fact[n] * self.fact_inv[k] * self.fact_inv[n - k]
    }
}

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
