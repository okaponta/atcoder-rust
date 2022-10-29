use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut map = HashMap::new();
    println!("{}", f(n, &mut map));
}

fn f(k: usize, map: &mut HashMap<usize, usize>) -> usize {
    if k == 0 {
        return 1;
    }
    if map.contains_key(&k) {
        return map[&k];
    }
    let res = f(k / 2, map) + f(k / 3, map);
    map.insert(k, res);
    res
}
