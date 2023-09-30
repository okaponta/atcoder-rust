use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        p:usize,
        ca:[(usize,[usize;k]);n],
    }
    let mut map = HashMap::new();
    map.insert(vec![0; k], 0);
    for (c, a) in ca {
        let mut new = map.clone();
        for (mut key, v) in map {
            for i in 0..k {
                key[i] = (key[i] + a[i]).min(p);
            }
            if !new.contains_key(&key) {
                new.insert(key, v + c);
            } else if v + c < new[&key] {
                new.insert(key, v + c);
            }
        }
        map = new;
    }
    if map.contains_key(&vec![p; k]) {
        println!("{}", map[&vec![p; k]]);
    } else {
        println!("-1");
    }
}
