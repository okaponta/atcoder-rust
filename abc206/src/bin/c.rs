use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
       n:i64,a:[i64;n],
    }
    let mut map: HashMap<i64, i64> = HashMap::new();
    for ai in a {
        if map.contains_key(&ai) {
            let before = *map.get(&ai).unwrap();
            map.insert(ai, before + 1);
        } else {
            map.insert(ai, 1);
        }
    }
    let ans = map
        .values()
        .into_iter()
        .map(|a| (a * (a - 1)) / 2)
        .fold((n * (n - 1)) / 2, |acc, x| acc - x);
    println!("{}", ans);
}
