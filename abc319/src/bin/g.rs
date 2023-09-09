use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut rev_g = vec![HashSet::new(); n];
    for (u, v) in uv {
        rev_g[u].insert(v);
        rev_g[v].insert(u);
    }
    let mut unvisited = (0..n).into_iter().collect::<HashSet<_>>();
    unvisited.remove(&0);
    let mut dist = vec![1 << 60; n];
    dist[0] = 0usize;
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let mut max = 0;
    while let Some((cur, d)) = q.pop_front() {
        let mut visited = vec![];
        for &next in &unvisited {
            if !rev_g[cur].contains(&next) {
                visited.push(next);
                dist[next] = d + 1;
                q.push_back((next, d + 1));
                max = d + 1;
            }
        }
        for u in visited {
            unvisited.remove(&u);
        }
    }
    if dist[n - 1] == 1 << 60 {
        println!("-1");
        return;
    }

    let mut edges = vec![vec![]; max + 1];
    for i in (0..n).rev() {
        if dist[i] == 1 << 60 {
            continue;
        }
        edges[dist[i]].push(i);
    }
    let mut dp = vec![ModInt::zero(); n];
    dp[0] = ModInt::one();
    let mut sum = ModInt::one();
    let mut prev = HashSet::new();
    prev.insert(0);
    for i in 1..=max {
        let mut s = ModInt::zero();
        let mut cur = HashSet::new();
        for &u in &edges[i] {
            let mut tmp = sum;
            for &ng in &rev_g[u] {
                if prev.contains(&ng) {
                    tmp -= dp[ng];
                }
            }
            dp[u] = tmp;
            s += tmp;
            cur.insert(u);
        }
        sum = s;
        prev = cur;
    }
    println!("{}", dp[n - 1].val);
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
