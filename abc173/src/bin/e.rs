use proconio::input;

fn main() {
    input! {
       n:usize,
       mut k:usize,
       a:[i64;n],
    }
    let mut a1 = vec![];
    let mut a2 = vec![];
    for a in a {
        if 0 <= a {
            a1.push(a as usize);
        } else {
            a2.push(-a as usize);
        }
    }
    a1.sort();
    a2.sort();
    if a1.len() == 0 {
        // 全て負
        if k % 2 == 0 {
            a2.reverse();
        }
        let ans = (0..k).fold(ModInt::one(), |s, i| {
            ModInt::zero() - s * ModInt::new(a2[i])
        });
        println!("{}", ans.val);
        return;
    }
    if n == k {
        // 全て選ぶ
        let one = a1
            .into_iter()
            .fold(ModInt::one(), |s, i| s * ModInt::new(i));
        let two = a2
            .into_iter()
            .fold(ModInt::one(), |s, i| ModInt::zero() - s * ModInt::new(i));
        println!("{}", (one * two).val);
        return;
    }
    a1.reverse();
    a2.reverse();
    let mut i1 = 0;
    let mut i2 = 0;
    let mut ans = ModInt::one();
    if k % 2 == 1 {
        i1 = 1;
        ans *= ModInt::new(a1[0]);
        k -= 1;
    }
    while 0 < k {
        if i1 + 1 < a1.len() && i2 + 1 < a2.len() {
            let m1 = a1[i1] * a1[i1 + 1];
            let m2 = a2[i2] * a2[i2 + 1];
            if m2 < m1 {
                ans *= ModInt::new(m1);
                i1 += 2;
            } else {
                ans *= ModInt::new(m2);
                i2 += 2;
            }
        } else if i1 + 1 < a1.len() {
            ans *= ModInt::new(a1[i1] * a1[i1 + 1]);
            i1 += 2;
        } else {
            ans *= ModInt::new(a2[i2] * a2[i2 + 1]);
            i2 += 2;
        }
        k -= 2;
    }
    println!("{}", ans.val);
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
