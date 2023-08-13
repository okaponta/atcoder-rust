use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        pq:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; 2 * n - 1];
    let mut prob = vec![ModInt::zero(); 2 * n - 1];
    let mut uf = UnionFind::new(2 * n - 1);
    let mut uf2 = UnionFind::new(n);
    for (i, (p, q)) in pq.into_iter().enumerate() {
        let psize = uf2.size(p);
        let qsize = uf2.size(q);
        let inv = ModInt::new(psize + qsize).inv();
        let probp = ModInt::new(psize) * inv;
        let probq = ModInt::new(qsize) * inv;
        uf2.union(p, q);

        let proot = uf.root(p);
        let qroot = uf.root(q);
        prob[proot] = probp;
        prob[qroot] = probq;

        g[i + n].push(proot);
        g[i + n].push(qroot);
        uf.union(i + n, proot);
        uf.union(i + n, qroot);
    }
    let mut ans = vec![ModInt::zero(); n];
    dfs(2 * n - 2, &g, &prob, &mut ans, n, ModInt::zero());
    println!("{}", ans.into_iter().map(|i| i.val).join(" "));
}

fn dfs(
    cur: usize,
    g: &Vec<Vec<usize>>,
    prob: &Vec<ModInt>,
    ans: &mut Vec<ModInt>,
    n: usize,
    mut p: ModInt,
) {
    if cur < n {
        ans[cur] = p;
        return;
    }
    for &next in &g[cur] {
        p += prob[next];
        dfs(next, g, prob, ans, n, p);
        p -= prob[next];
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            rank: vec![0; n],
        }
    }

    // 根を返却
    pub fn root(&mut self, x: usize) -> usize {
        // parentが自分自身の場合は根
        if self.parent[x] == x {
            return x;
        }
        // 経路圧縮
        self.parent[x] = self.root(self.parent[x]);
        self.parent[x]
    }

    // xとyが同じ根か判定
    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // xとyを合体
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        // 既に同じ
        if rx == ry {
            return false;
        }

        // ryのrankが小さくなるように調整
        // ここを省略するとrxが親になる
        // if self.rank[rx] < self.rank[ry] {
        //     std::mem::swap(&mut rx, &mut ry);
        // }
        // ryの根をrxにする
        self.parent[ry] = rx;
        // rxのrank調整
        if self.rank[rx] == self.rank[ry] {
            self.rank[rx] += 1;
        }
        self.size[rx] += self.size[ry];
        true
    }

    // xのグループの要素数
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.size[root]
    }

    // 連結かどうかを返却する
    pub fn is_linked(&mut self) -> bool {
        self.size(0) == self.size.len()
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
