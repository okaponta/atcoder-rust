use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let n = s.len();
    let mut map = HashMap::new();
    for i in 0..n {
        if map.contains_key(&s[i]) {
            if t[i] != map[&s[i]] {
                println!("No");
                return;
            }
        }
        map.insert(s[i], t[i]);
    }
    let key_num = map.keys().len();
    let value_num = map.values().unique_by(|&v| v).count();
    println!("{}", if key_num == value_num { "Yes" } else { "No" });
}
