use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        r:usize,
        g:usize,
        b:usize,
        k:usize,
        x:usize,
        y:usize,
        z:usize,
    }
    let n = 200010;
    let comb = Binom::init(n, MOD);
    let mut cr = vec![ModInt::zero(); r + 1];
    let mut cg = vec![ModInt::zero(); g + 1];
    // 赤の場合の数
    for i in k - y..=r {
        cr[i].val = comb.get(r, i);
    }
    // 緑の場合の数
    for i in k - z..=g {
        cg[i].val = comb.get(g, i);
    }
    // 畳み込みで赤と緑の合計の場合の数を求める
    let crg = convolve(&cr, &cg);

    let mut ans = ModInt::zero();
    // 青の場合と、畳み込みで求めた赤と緑の場合を掛け算して答えを求める
    for i in k - x..=b {
        if i <= k {
            ans += crg[k - i] * ModInt::new(comb.get(b, i));
        }
    }
    println!("{}", ans.val);
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
        assert!(n >= k);
        self.fact[n] * self.fact_inv[k] % self.modulo * self.fact_inv[n - k] % self.modulo
    }
}

fn convolve(a: &Vec<ModInt>, b: &Vec<ModInt>) -> Vec<ModInt> {
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

    let mut a = a.iter().copied().collect::<Vec<_>>();
    let mut b = b.iter().copied().collect::<Vec<_>>();
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
