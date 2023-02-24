use proconio::{input, marker::Usize1};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
       n:usize,
       uvw:[(Usize1,Usize1,usize);n-1],
    }
    let mut ans = ModInt::zero();
    let mut pow2 = ModInt::one();
    let mut edges = vec![vec![]; n];
    for (u, v, w) in uvw {
        edges[u].push((v, w));
        edges[v].push((u, w));
    }
    let mut dist = vec![vec![0; n]; 60];
    dfs_cost(0, 0, &edges, &mut dist);
    for i in 0..60 {
        let s = dist[i].iter().sum::<usize>();
        ans += ModInt::new(s) * ModInt::new(n - s) * pow2;
        pow2 *= ModInt::new(2);
    }
    println!("{}", ans.val);
}

fn dfs_cost(prev: usize, cur: usize, edges: &Vec<Vec<(usize, usize)>>, d: &mut Vec<Vec<usize>>) {
    for &(next, cost) in &edges[cur] {
        if next == prev {
            continue;
        }
        for i in 0..60 {
            d[i][next] = d[i][cur] ^ cost >> i & 1;
        }
        dfs_cost(cur, next, edges, d);
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
