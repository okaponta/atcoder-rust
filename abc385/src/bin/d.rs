use std::collections::*;

use amplify::confinement::Collection;
use proconio::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut x:i64,
        mut y:i64,
        xy:[(i64,i64);n],
        dc:[(char,i64);m],
    }
    let mut ud_map = HashMap::new();
    let mut lr_map = HashMap::new();
    for (x, y) in xy {
        ud_map.entry(x).or_insert(BTreeSet::new()).push(y);
        lr_map.entry(y).or_insert(BTreeSet::new()).push(x);
    }
    let mut cnt = 0;
    for (d, c) in dc {
        let (dx, dy) = match d {
            'U' => (0, c),
            'D' => (0, -c),
            'L' => (-c, 0),
            _ => (c, 0),
        };
        let mut tmp = vec![];
        if dy == 0 {
            if let Some(set) = lr_map.get_mut(&y) {
                for nx in set.range(x.min(x + dx)..=x.max(x + dx)) {
                    tmp.push((*nx, y));
                }
            }
        } else {
            if let Some(set) = ud_map.get_mut(&x) {
                for ny in set.range(y.min(y + dy)..=y.max(y + dy)) {
                    tmp.push((x, *ny));
                }
            }
        }
        for (x, y) in tmp {
            cnt += 1;
            ud_map.get_mut(&x).unwrap().remove(&y);
            lr_map.get_mut(&y).unwrap().remove(&x);
        }
        x += dx;
        y += dy;
    }
    println!("{} {} {}", x, y, cnt);
}
