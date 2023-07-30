use std::collections::HashSet;

use itertools::iproduct;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        xyz:[(usize,usize,usize,usize,usize,usize);n],
    }
    let mut m = vec![vec![vec![0; 100]; 100]; 100];
    for (i, (x1, y1, z1, x2, y2, z2)) in xyz.into_iter().enumerate() {
        for (j, k, l) in iproduct!(x1..x2, y1..y2, z1..z2) {
            m[j][k][l] = i + 1;
        }
    }
    let mut ans = vec![HashSet::new(); n + 1];
    for (i, j, k) in iproduct!(0..100, 0..100, 0..100) {
        let cur = m[i][j][k];
        if cur == 0 {
            continue;
        }
        for (dx, dy, dz) in vec![
            (!0, 0, 0),
            (1, 0, 0),
            (0, !0, 0),
            (0, 1, 0),
            (0, 0, !0),
            (0, 0, 1),
        ] {
            let nx = i.wrapping_add(dx);
            let ny = j.wrapping_add(dy);
            let nz = k.wrapping_add(dz);
            if nx < 100 && ny < 100 && nz < 100 {
                let next = m[nx][ny][nz];
                if next != 0 && cur != next {
                    ans[cur].insert(next);
                    ans[next].insert(cur);
                }
            }
        }
    }
    ans.remove(0);
    for set in ans {
        println!("{}", set.len());
    }
}
