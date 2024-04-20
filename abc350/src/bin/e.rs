use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        x:f64,
        y:f64,
    }
    let mut map = HashMap::new();
    map.insert(0, 0.0);
    let ans = dfs(n, a, x, y, &mut map);
    println!("{}", ans);
}

fn dfs(cur: usize, a: usize, x: f64, y: f64, map: &mut HashMap<usize, f64>) -> f64 {
    if map.contains_key(&cur) {
        return map[&cur];
    }
    let tmp1 = dfs(cur / a, a, x, y, map) + x;
    let tmp2 = (dfs(cur / 2, a, x, y, map)
        + dfs(cur / 3, a, x, y, map)
        + dfs(cur / 4, a, x, y, map)
        + dfs(cur / 5, a, x, y, map)
        + dfs(cur / 6, a, x, y, map))
        / 5.0
        + y * 6.0 / 5.0;
    if tmp1 < tmp2 {
        map.insert(cur, tmp1);
        return tmp1;
    } else {
        map.insert(cur, tmp2);
        return tmp2;
    }
}
