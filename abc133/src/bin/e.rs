use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        k:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    if k == 1 {
        println!("{}", if n == 1 { 1 } else { 0 });
        return;
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut fact = vec![ModInt::one()];
    for i in 1..=k {
        fact.push(fact[i - 1] * ModInt::new(i));
    }
    println!("{}", dfs(0, 0, &g, &fact, k).val);
}

fn dfs(cur: usize, prev: usize, g: &Vec<Vec<usize>>, fact: &Vec<ModInt>, k: usize) -> ModInt {
    let mut res = if cur == 0 {
        mul(k, g[0].len() + 1, fact)
    } else {
        mul(k - 2, g[cur].len() - 1, fact)
    };
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        res *= dfs(next, cur, g, fact, k);
    }
    res
}

// nからr個ぶんだけ1ひきながらかける
fn mul(n: usize, r: usize, fact: &Vec<ModInt>) -> ModInt {
    if n < r {
        return ModInt::zero();
    }
    fact[n] * fact[n - r].inv()
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
