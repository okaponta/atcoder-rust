use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut grid = vec![vec![0; w]; h];
    let mut q = VecDeque::new();
    let mut low_link = LowLink::new(h * w);
    let mut single = vec![];
    let mut count = 0;
    let mut green = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                green += 1;
            }
            if s[i][j] != '#' || grid[i][j] != 0 {
                continue;
            }
            count += 1;
            q.push_back((i, j, count));
            grid[i][j] = count;
            while let Some((x, y, count)) = q.pop_front() {
                let mut sur = 0;
                for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                    let xi = x.wrapping_add(dx);
                    let yi = y.wrapping_add(dy);
                    if h <= xi || w <= yi {
                        continue;
                    }
                    if s[xi][yi] == '#' {
                        sur += 1;
                        low_link.add_edges(x * w + y, xi * w + yi);
                        if grid[xi][yi] == 0 {
                            q.push_back((xi, yi, count));
                            grid[xi][yi] = count;
                        }
                    }
                }
                if sur == 0 {
                    single.push(i * w + j);
                }
            }
        }
    }
    low_link.execute();
    let mut sum = count * green;
    sum -= single.len();
    sum += low_link
        .articulation_points
        .iter()
        .map(|p| p.1 - 1)
        .sum::<usize>();
    let ans = ModInt::new(sum) * ModInt::new(green).inv();
    println!("{}", ans.val);
}

pub struct LowLink {
    n: usize,
    g: Vec<Vec<usize>>,
    articulation_points: Vec<(usize, usize)>,
    bridges: Vec<(usize, usize)>,
}

impl LowLink {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
            articulation_points: vec![],
            bridges: vec![],
        }
    }

    pub fn add_edges(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
    }

    pub fn execute(&mut self) -> usize {
        let mut ord = vec![0; self.n];
        let mut low = vec![0; self.n];
        let mut bridges = vec![];
        let mut articulation = vec![];
        fn dfs(
            cur: usize,
            prev: usize,
            mut count: usize,
            g: &Vec<Vec<usize>>,
            used: &mut Vec<bool>,
            ord: &mut Vec<usize>,
            low: &mut Vec<usize>,
            articulation: &mut Vec<(usize, usize)>,
            bridges: &mut Vec<(usize, usize)>,
        ) -> usize {
            used[cur] = true;
            count += 1;
            ord[cur] = count;
            low[cur] = ord[cur];
            let mut articulation_count = 0;
            let mut childs = 0;
            for &next in &g[cur] {
                if !used[next] {
                    childs += 1;
                    count = dfs(next, cur, count, g, used, ord, low, articulation, bridges);
                    low[cur] = low[cur].min(low[next]);
                    if ord[cur] <= low[next] {
                        articulation_count += 1;
                    }
                    if ord[cur] < low[next] {
                        bridges.push((cur, next));
                    }
                } else if next != prev {
                    low[cur] = low[cur].min(ord[next]);
                }
            }
            if prev == !0 {
                if 2 <= childs {
                    articulation.push((cur, articulation_count));
                }
            } else if 0 < articulation_count {
                articulation.push((cur, articulation_count + 1));
            }
            count
        }
        let mut used = vec![false; self.n];
        for i in 0..self.n {
            if !used[i] {
                dfs(
                    i,
                    !0,
                    0,
                    &self.g,
                    &mut used,
                    &mut ord,
                    &mut low,
                    &mut articulation,
                    &mut bridges,
                );
            }
        }
        self.bridges = bridges;
        self.articulation_points = articulation;
        self.bridges.len()
    }
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

impl std::fmt::Debug for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)?;
        Ok(())
    }
}
