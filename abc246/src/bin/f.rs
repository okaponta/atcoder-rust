use itertools::Itertools;
use num::PrimInt;
use proconio::{input, marker::Chars};

const MOD: i64 = 998244353;

fn main() {
    input! {
        n:usize,l:i64,
        s:[Chars;n],
    }
    let mut sint = vec![0; n];
    for i in 0..n {
        for &c in &s[i] {
            sint[i] += 1 << (c as u8 - 'a' as u8);
        }
    }
    let pows = (0..27).into_iter().map(|i| pow(i, l, MOD)).collect_vec();
    let mut ans = 0;
    for i in 1..1 << n {
        let mut a = (1 << 26) - 1;
        let mut cnt = 0;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                a &= sint[j];
                cnt += 1;
            }
        }
        if cnt % 2 == 1 {
            ans += pows[a.count_ones() as usize];
        } else {
            ans -= pows[a.count_ones() as usize];
        }
        ans = (ans + MOD) % MOD;
    }
    println!("{}", ans);
}

fn pow(mut x: i64, mut n: i64, modulo: i64) -> i64 {
    x %= modulo;
    let mut ret = if x == 0 { 0 } else { 1 };
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % modulo;
        }
        x = x * x % modulo;
        n >>= 1;
    }
    ret
}
