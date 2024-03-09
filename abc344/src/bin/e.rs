use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        q:usize
    }
    let mut map = HashMap::new();
    for i in 0..n {
        if n == 1 {
            map.insert(a[i], (0, 0));
        } else if i == 0 {
            map.insert(a[i], (0, a[i + 1]));
        } else if i == n - 1 {
            map.insert(a[i], (a[i - 1], 0));
        } else {
            map.insert(a[i], (a[i - 1], a[i + 1]));
        }
    }
    for _ in 0..q {
        input! {c:u8}
        if c == 1 {
            input! {x:usize, y:usize}
            let &(_, after) = map.get(&x).unwrap();
            map.entry(x).or_insert((0, 0)).1 = y;
            if after != 0 {
                map.entry(after).or_insert((0, 0)).0 = y;
            }
            map.insert(y, (x, after));
        } else {
            input! {x:usize}
            let (bef, after) = map.remove(&x).unwrap();
            if bef == 0 {
                map.entry(after).or_insert((0, 0)).0 = bef;
            } else if after == 0 {
                map.entry(bef).or_insert((0, 0)).1 = after;
            } else {
                map.entry(bef).or_insert((0, 0)).1 = after;
                map.entry(after).or_insert((0, 0)).0 = bef;
            }
        }
    }

    let mut start = 0;
    for (k, v) in &map {
        if v.0 == 0 {
            start = *k;
        }
    }

    let mut ans = vec![start];
    let mut tmp = start;
    loop {
        let &(_, next) = map.get(&tmp).unwrap();
        if next == 0 {
            break;
        }
        ans.push(next);
        tmp = next;
    }
    println!("{}", ans.iter().join(" "));
}
