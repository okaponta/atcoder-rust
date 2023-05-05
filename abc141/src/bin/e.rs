use std::collections::HashSet;

use proconio::{input, marker::Chars};

const BASE: u128 = 1000001137;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let roliha = RollingHash::init(BASE, n, &s);
    // 長さ
    let mut lower = 0;
    let mut upper = n / 2 + 1;
    while 1 < upper - lower {
        let mid = (upper + lower) / 2;
        let mut is_ok = false;
        let mut set = HashSet::new();
        // 開始位置
        for j in (0..=n - 2 * mid).rev() {
            let h1 = roliha.hash(j, j + mid);
            // 開始位置(一致する方)
            let h2 = roliha
                .hash(j + mid, j + mid * 2)
                .wrapping_mul(roliha.b[n - 2 * mid - j]);
            set.insert(h2);
            if set.contains(&(h1.wrapping_mul(roliha.b[n - mid - j]))) {
                is_ok = true;
                break;
            }
        }
        if is_ok {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    println!("{}", lower);
}

pub struct RollingHash {
    b: Vec<u128>,
    hash: Vec<u128>,
}

impl RollingHash {
    // const BASE:u128 = 1000001137;
    pub fn init(base: u128, n: usize, str: &Vec<char>) -> Self {
        let mut b = vec![1u128];
        let mut hash = vec![0u128];
        let mut t = 1;
        for i in 0..n {
            hash.push((str[i] as u128).wrapping_mul(t).wrapping_add(hash[i]));
            t = t.wrapping_mul(base);
            b.push(t);
        }
        Self { b, hash }
    }

    // i..jまでの文字列のハッシュ値
    // 位置が違うところで比較するときは、b[差分]だけかけてから比較すること
    fn hash(&self, i: usize, j: usize) -> u128 {
        self.hash[j].wrapping_sub(self.hash[i])
    }
}
