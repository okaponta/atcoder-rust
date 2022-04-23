use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        k:usize,
        s:[Chars;n],
    }
    let mut sets = vec![];
    for ch in s {
        // 計算量落とすためにsetで管理した
        let set = ch.into_iter().collect::<HashSet<_>>();
        sets.push(set);
    }
    let mut ans = 0;
    // bit全探索
    for i in 0..1 << n {
        // A-Zまでの登場回数を記録
        let mut count = vec![0; 26];
        for j in 0..n {
            // bitが立っていれば処理する
            if i >> j & 1 == 1 {
                // A-Zそれぞれについて含まれるか計算
                for k in 0..26 {
                    let ch = (b'a' + k as u8) as char;
                    if sets[j].contains(&ch) {
                        count[k] += 1;
                    }
                }
            }
        }
        let tmp = count.iter().filter(|i| i == &&k).count();
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
