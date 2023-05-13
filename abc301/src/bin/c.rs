use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let at = vec!['a', 't', 'c', 'o', 'd', 'e', 'r']
        .into_iter()
        .collect::<HashSet<char>>();
    let mut map = HashMap::new();
    let mut aster = 0;
    for c in s {
        if c == '@' {
            aster += 1;
            continue;
        }
        *map.entry(c).or_insert(0i64) += 1;
    }
    for c in t {
        if c == '@' {
            aster += 1;
            continue;
        }
        *map.entry(c).or_insert(0) -= 1;
    }
    for (k, v) in map {
        if at.contains(&k) {
            aster -= v.abs();
        } else {
            if v != 0 {
                println!("No");
                return;
            }
        }
    }
    println!("{}", if 0 <= aster { "Yes" } else { "No" });
}
