use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
       n:usize,
       a:[i64;n],
    }
    let mut map = HashMap::new();
    let mut count: i64 = 0;
    for i in 0..n {
        count += map.get(&(a[i] - i as i64)).map_or(0, |v| *v);
        *map.entry(-a[i] - i as i64).or_insert(0) += 1;
    }
    println!("{}", count);
}
