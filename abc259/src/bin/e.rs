use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut num = vec![];
    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    for _ in 0..n {
        input! {
            m:usize,
            pe:[(usize,usize);m],
        }
        let mut set = HashSet::new();
        for (p, e) in pe {
            set.insert((p, e));
            if map.contains_key(&p) {
                let bef = *map.get(&p).unwrap();
                if bef.0 < e {
                    map.insert(p, (e, bef.0));
                } else if bef.1 < e {
                    map.insert(p, (bef.0, e));
                }
            } else {
                map.insert(p, (e, 0));
            }
        }
        num.push(set);
    }
    let mut ans = 0;
    for set in num {
        for (p, e) in set {
            let v = map.get(&p).unwrap();
            if v.0 == e && v.0 != v.1 {
                ans += 1;
                break;
            }
        }
    }
    if ans < n {
        ans += 1;
    }
    println!("{}", ans);
}
