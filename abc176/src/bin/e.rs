use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _:usize,_:usize,m:usize,
        hw:[(Usize1,Usize1);m],
    }
    let mut hmap = HashMap::new();
    let mut wmap = HashMap::new();
    for (h, w) in hw.clone() {
        *hmap.entry(h).or_insert(0i64) += 1;
        *wmap.entry(w).or_insert(0i64) += 1;
    }
    let hmax = *hmap.values().max().unwrap();
    let wmax = *wmap.values().max().unwrap();
    let hkey = hmap
        .iter()
        .filter(|&e| e.1 == &hmax)
        .map(|e| *e.0)
        .collect::<HashSet<_>>();
    let wkey = wmap
        .iter()
        .filter(|&e| e.1 == &wmax)
        .map(|e| *e.0)
        .collect::<HashSet<_>>();
    let mut ans = hmax + wmax;
    let mut count = hkey.len() * wkey.len();
    for (h, w) in hw {
        if hkey.contains(&h) && wkey.contains(&w) {
            count -= 1;
        }
    }
    if count == 0 {
        ans -= 1;
    }
    println!("{}", ans);
}
