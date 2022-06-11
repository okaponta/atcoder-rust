use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,m:usize,
        s:[i64;n-1],
        x:[i64;m],
    }
    let mut a = vec![0];
    for i in 0..n - 1 {
        a.push(s[i] - a[i]);
    }
    let mut map = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            if i % 2 == 0 {
                *map.entry(x[j] - a[i]).or_insert(0) += 1;
            } else {
                *map.entry(a[i] - x[j]).or_insert(0) += 1;
            }
        }
    }
    println!("{}", map.values().max().unwrap());
}
