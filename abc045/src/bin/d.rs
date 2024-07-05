use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
        ab:[(Usize1,Usize1);n],
    }
    let mut map = HashMap::new();
    for (a, b) in ab {
        let dx = vec![0, !0, !0, 0, 1, 1, 1, 0, !0];
        let dy = vec![0, 0, 1, 1, 1, 0, !0, !0, !0];
        for i in 0..9 {
            let xi = a.wrapping_add(dx[i]);
            let yi = b.wrapping_add(dy[i]);
            if xi == 0 || yi == 0 || h - 1 <= xi || w - 1 <= yi {
                continue;
            }
            *map.entry((xi, yi)).or_insert(0) += 1;
        }
    }
    let mut ans = vec![0; 10];
    ans[0] = (h - 2) * (w - 2);
    for (_, v) in map {
        ans[v] += 1;
        ans[0] -= 1;
    }
    for i in 0..10 {
        println!("{}", ans[i]);
    }
}
