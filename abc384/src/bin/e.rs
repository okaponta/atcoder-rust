use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        h:usize,
        w:usize,
        x:u128,
        p:Usize1,
        q:Usize1,
        s:[[u128;w];h],
    }
    let mut ans = s[p][q];
    let mut heap = BinaryHeap::new();
    let mut used = vec![vec![false; w]; h];
    for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
        let ni = p.wrapping_add(di);
        let nj = q.wrapping_add(dj);
        if h <= ni || w <= nj {
            continue;
        }
        heap.push(Reverse((s[ni][nj], ni, nj)));
        used[ni][nj] = true;
    }
    used[p][q] = true;
    while let Some(Reverse((p, i, j))) = heap.pop() {
        if p * x < ans {
            ans += p;
            used[i][j] = true;
            for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if h <= ni || w <= nj {
                    continue;
                }
                if used[ni][nj] {
                    continue;
                }
                heap.push(Reverse((s[ni][nj], ni, nj)));
                used[ni][nj] = true;
            }
        } else {
            break;
        }
    }

    println!("{}", ans);
}
