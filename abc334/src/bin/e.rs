use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut grid = vec![vec![0; w]; h];
    let mut q = VecDeque::new();
    let mut count = 1;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            if grid[i][j] != 0 {
                continue;
            }
            q.push_back((i, j, count));
            grid[i][j] = count;
            while let Some((x, y, count)) = q.pop_front() {
                for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                    let xi = x.wrapping_add(dx);
                    let yi = y.wrapping_add(dy);
                    if h <= xi || w <= yi {
                        continue;
                    }
                    if s[xi][yi] == '#' && grid[xi][yi] == 0 {
                        q.push_back((xi, yi, count));
                        grid[xi][yi] = count;
                    }
                }
            }
            count += 1;
        }
    }
    count -= 1;
    let mut sum = 0;
    let mut div = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut sur = HashSet::new();
            for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                let xi = i.wrapping_add(dx);
                let yi = j.wrapping_add(dy);
                if h <= xi || w <= yi {
                    continue;
                }
                if s[xi][yi] == '#' {
                    sur.insert(grid[xi][yi]);
                }
            }
            if sur.len() == 0 {
                sum += count + 1;
            } else {
                sum += count - (sur.len() - 1);
            }
            div += 1;
        }
    }
    let ans = ModInt::new(sum) * ModInt::new(div).inv();
    println!("{}", ans.val);
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
