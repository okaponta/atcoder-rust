use std::collections::HashSet;

use proconio::{input, marker::Chars};

const BASE: u128 = 1000001137;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let b = BASE;
    let mut t = 1;
    // ローリングハッシュを累積和みたいな感じで管理
    let mut cum = vec![0];
    let mut base = vec![t];
    for i in 0..n {
        cum.push((s[i] as u128).wrapping_mul(t).wrapping_add(cum[i]));
        t = t.wrapping_mul(b);
        base.push(t);
    }
    // 長さ
    for i in (0..=n / 2).rev() {
        let mut set = HashSet::new();
        // 開始位置
        for j in (0..=n - 2 * i).rev() {
            let h1 = cum[j + i].wrapping_sub(cum[j]);
            // 開始位置(一致する方)
            let h2 = (cum[j + 2 * i].wrapping_sub(cum[j + i])).wrapping_mul(base[n - 2 * i - j]);
            set.insert(h2);
            if set.contains(&(h1.wrapping_mul(base[n - i - j]))) {
                println!("{}", i);
                return;
            }
        }
    }
}
