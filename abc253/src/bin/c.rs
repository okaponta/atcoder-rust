use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q:usize,
    }
    let mut map = BTreeMap::new();
    for _ in 0..q {
        input! {query:u8}
        if query == 1 {
            input! {x:usize}
            *map.entry(x).or_insert(0usize) += 1;
        } else if query == 2 {
            input! {x:usize,c:usize}
            if let Some(before) = map.get(&x) {
                let after = before.saturating_sub(c);
                if after == 0 {
                    map.remove(&x);
                } else {
                    map.insert(x, after);
                }
            }
        } else {
            let min = *map.iter().next().unwrap().0;
            let max = *map.iter().last().unwrap().0;
            println!("{}", max - min);
        }
    }
}
