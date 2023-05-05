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
    let mut lower = 0;
    let mut upper = n / 2 + 1;
    while 1 < upper - lower {
        let mid = (upper + lower) / 2;
        let mut is_ok = false;
        let mut set = HashSet::new();
        // 開始位置
        for j in (0..=n - 2 * mid).rev() {
            let h1 = cum[j + mid].wrapping_sub(cum[j]);
            // 開始位置(一致する方)
            let h2 =
                (cum[j + 2 * mid].wrapping_sub(cum[j + mid])).wrapping_mul(base[n - 2 * mid - j]);
            set.insert(h2);
            if set.contains(&(h1.wrapping_mul(base[n - mid - j]))) {
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
