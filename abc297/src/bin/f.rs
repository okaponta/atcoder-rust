use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        h:usize,
        w:usize,
        k:usize,
    }
    let comb = Binom::init(h * w, MOD);
    let div = ModInt::new(comb.get(h * w, k)).inv();
    let mut ans = ModInt::zero();
    for i in 0..h {
        for j in 0..w {
            ans += ModInt::one() - calc(i, h - i - 1, j, w - j - 1, &comb, h, w, k) * div;
        }
    }
    println!("{}", ans.val);
}

fn calc(
    h1: usize,
    h2: usize,
    w1: usize,
    w2: usize,
    comb: &Binom,
    h: usize,
    w: usize,
    k: usize,
) -> ModInt {
    ModInt::new(comb.get(h * w1, k))
        + ModInt::new(comb.get(h * w2, k))
        + ModInt::new(comb.get(w * h1, k))
        + ModInt::new(comb.get(w * h2, k))
        - ModInt::new(comb.get(h1 * w1, k))
        - ModInt::new(comb.get(h1 * w2, k))
        - ModInt::new(comb.get(h2 * w1, k))
        - ModInt::new(comb.get(h2 * w2, k))
}

pub struct Binom {
    fact: Vec<usize>,
    fact_inv: Vec<usize>,
    modulo: usize,
}

impl Binom {
    pub fn init(n: usize, modulo: usize) -> Self {
        let mut fact = vec![0; n + 1];
        let mut inv = vec![0; n + 1];
        let mut fact_inv = vec![0; n + 1];
        fact[0] = 1;
        fact[1] = 1;
        inv[1] = 1;
        fact_inv[0] = 1;
        fact_inv[1] = 1;
        for i in 2..(n + 1) {
            fact[i] = fact[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            fact_inv[i] = fact_inv[i - 1] * inv[i] % modulo;
        }
        Self {
            fact,
            fact_inv,
            modulo,
        }
    }

    // nCk
    pub fn get(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        assert!(n >= k);
        self.fact[n] * self.fact_inv[k] % self.modulo * self.fact_inv[n - k] % self.modulo
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
