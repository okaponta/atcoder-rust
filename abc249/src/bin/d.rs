use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    // key...数字,value...登場回数
    let mut map = HashMap::new();
    let mut max = 0;
    for ai in a {
        max = max.max(ai);
        *map.entry(ai).or_insert(0i64) += 1;
    }
    let mut ans = 0;
    // ajは全探索
    for (&aj, val) in map.iter() {
        // akは1からMAXまで
        for ak in 1..=(max / aj) {
            let ai = aj * ak;
            // 登場回数どうしを掛け算。無い場合は0。
            ans += map.get(&ai).unwrap_or(&0) * map.get(&ak).unwrap_or(&0) * val;
        }
    }
    println!("{}", ans);
}
