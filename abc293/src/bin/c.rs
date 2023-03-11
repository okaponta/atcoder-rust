use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[usize;w];h],
    }
    let mut ans = 0;
    for v in (0..h + w - 2).combinations(h - 1) {
        let mut set = HashSet::new();
        let mut cur = vec![0, 0];
        let mut step = 0;
        while cur != vec![h - 1, w - 1] {
            set.insert(a[cur[0]][cur[1]]);
            if v.contains(&step) {
                cur[0] += 1;
            } else {
                cur[1] += 1;
            }
            step += 1;
        }
        set.insert(a[cur[0]][cur[1]]);
        if set.len() == h + w - 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
