use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       p:[usize;3]
    }
    let mut map = HashMap::new();
    map.insert(p, 1.0);
    let mut ans = 0.0;
    let mut count = 0;
    while !map.is_empty() {
        let mut next = HashMap::new();
        for (p, v) in map {
            if p.iter().any(|e| e == &100) {
                ans += count as f64 * v;
                continue;
            }
            let sum = p[0] + p[1] + p[2];
            let prob = p.iter().map(|i| *i as f64 / sum as f64).collect_vec();
            for i in 0..3 {
                if p[i] != 0 {
                    let mut k = p.clone();
                    k[i] += 1;
                    *next.entry(k).or_insert(0.0) += v * prob[i];
                }
            }
        }
        map = next;
        count += 1;
    }
    println!("{}", ans);
}
