use std::collections::HashMap;

use num_bigint::BigInt;
use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[BigInt;n],
    }
    a.sort();
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i].clone()).or_insert(0) += 1;
    }
    let max = a[n - 1].clone();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            let mul = &a[i] * &a[j];
            if max < mul {
                break;
            }
            if let Some(&c) = map.get(&mul) {
                ans += c;
            }
        }
    }
    println!("{}", ans);
}
