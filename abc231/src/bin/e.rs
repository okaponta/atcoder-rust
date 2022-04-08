use std::collections::HashMap;

use num::Integer;
use proconio::input;

fn main() {
    input! {
       n:usize,
       x:usize,
       a:[usize;n],
    }
    println!("{}", dfs(x, &a, 0, &mut HashMap::new()));
}

fn dfs(x: usize, a: &Vec<usize>, i: usize, map: &mut HashMap<usize, usize>) -> usize {
    if map.contains_key(&x) {
        return map[&x];
    }
    if i == a.len() - 1 {
        return x / a[i];
    }
    let ls = (x / a[i + 1]) * a[i + 1];
    let ov = x.div_ceil(&a[i + 1]) * a[i + 1];
    let less = (x - ls) / a[i] + dfs(ls, a, i + 1, map);
    let over = (ov - x) / a[i] + dfs(ov, a, i + 1, map);
    map.insert(x, over.min(less));
    over.min(less)
}
