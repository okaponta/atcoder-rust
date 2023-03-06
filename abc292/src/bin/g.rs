use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
    }
    // mem[l][r][k] Sl..Srで上からk+1桁目以降が条件に合致
    let mut mem = vec![vec![vec![ModInt::zero(); m + 1]; n + 1]; n + 1];
    let mut done = vec![vec![vec![false; m + 1]; n + 1]; n + 1];
    println!("{}", f(0, n, 0, m, n, &mut mem, &mut done, &s).val);
}

fn f(
    l: usize,
    r: usize,
    k: usize,
    m: usize,
    n: usize,
    mem: &mut Vec<Vec<Vec<ModInt>>>,
    done: &mut Vec<Vec<Vec<bool>>>,
    s: &Vec<Vec<char>>,
) -> ModInt {
    if l == r {
        return ModInt::one();
    }
    if k == m {
        // 最後の桁
        return ModInt::new(if r - l == 1 { 1 } else { 0 });
    }
    if done[l][r][k] {
        return mem[l][r][k];
    }
    done[l][r][k] = true;
    let mut dp = vec![ModInt::zero(); n + 1];
    dp[l] = ModInt::one();
    for d in 0..10 {
        let mut next = vec![ModInt::zero(); n + 1];
        for i in l..=r {
            for j in i..=r {
                next[j] += dp[i] * f(i, j, k + 1, m, n, mem, done, s);
                if j == r {
                    break;
                }
                if s[j][k] != '?' && s[j][k] != std::char::from_digit(d as u32, 10).unwrap() {
                    break;
                }
            }
        }
        dp = next;
    }
    mem[l][r][k] = dp[r];
    return mem[l][r][k];
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
