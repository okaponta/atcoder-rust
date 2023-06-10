use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;m],
    }
    let mut sb = vec![];
    for s in s {
        sb.push(to_bit('b', s));
    }
    if n <= 6 {
        println!("{}", all(n, &sb).len());
        return;
    }
    let mut first = vec![ModInt::zero(); 1 << 6];
    for i in all(6, &sb) {
        first[i] += ModInt::one();
    }

    let mut matrix = vec![vec![ModInt::zero(); 1 << 6]; 1 << 6];
    for i in 0..1 << 6 {
        let nxa = (i << 1) & ((1 << 6) - 1);
        if check(nxa, 6, &sb) {
            matrix[i][nxa] += ModInt::one();
        }
        let nxb = ((i << 1) + 1) & ((1 << 6) - 1);
        if check(nxb, 6, &sb) {
            matrix[i][nxb] += ModInt::one();
        }
    }
    let pw = pow(matrix, n - 6);
    let mut ans = vec![ModInt::zero(); 1 << 6];
    for i in 0..1 << 6 {
        for j in 0..1 << 6 {
            ans[j] += first[i] * pw[i][j];
        }
    }
    let mut sum = ModInt::zero();
    for i in 0..1 << 6 {
        sum += ans[i];
    }
    println!("{}", sum.val);
}

// bit表現したときの数字と文字列の長さを返却する
fn to_bit(one: char, target: Vec<char>) -> (usize, usize) {
    let mut res = 0;
    for i in 0..target.len() {
        if target[i] == one {
            res += 1 << (target.len() - 1 - i)
        }
    }
    (res, target.len())
}

fn all(n: usize, s: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut res = vec![];
    let mut one = vec![true, true];
    for &ban in s {
        if ban.1 == 1 {
            one[ban.0] = false;
        }
    }
    for i in 0..2 {
        if one[i] {
            res.push(i);
        }
    }
    for i in 1..n {
        let mut next = vec![];
        for &j in &res {
            let nxa = j << 1;
            if check(nxa, i + 1, &s) {
                next.push(nxa);
            }
            let nxb = (j << 1) + 1;
            if check(nxb, i + 1, &s) {
                next.push(nxb);
            }
        }
        res = next;
    }
    res
}

fn check(target: usize, len: usize, s: &Vec<(usize, usize)>) -> bool {
    for ban in s {
        if len < ban.1 {
            continue;
        }
        if target & ((1 << ban.1) - 1) == ban.0 {
            return false;
        }
    }
    true
}

fn pow(mut a: Vec<Vec<ModInt>>, mut n: usize) -> Vec<Vec<ModInt>> {
    let mut b = vec![vec![ModInt::zero(); a.len()]; a.len()];
    for i in 0..a.len() {
        b[i][i] = ModInt::one();
    }
    while 0 < n {
        if n & 1 == 1 {
            b = mul(&a, &b);
        }
        a = mul(&a, &a);
        n >>= 1;
    }
    b
}

fn mul(a: &Vec<Vec<ModInt>>, b: &Vec<Vec<ModInt>>) -> Vec<Vec<ModInt>> {
    let n = a.len();
    let mut res = vec![vec![ModInt::zero(); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    res
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
