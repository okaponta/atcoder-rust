use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        y:usize,
        a:[[usize;w];h],
    }
    let mut used = vec![vec![false; w]; h];
    let mut count = h * w;
    let mut heap = BinaryHeap::new();
    for i in 0..w {
        heap.push(Reverse((a[0][i], 0, i)));
        heap.push(Reverse((a[h - 1][i], h - 1, i)));
    }
    for i in 0..h {
        heap.push(Reverse((a[i][0], i, 0)));
        heap.push(Reverse((a[i][w - 1], i, w - 1)));
    }
    for y in 1..=y {
        while let Some(Reverse((aa, i, j))) = heap.pop() {
            if used[i][j] {
                continue;
            }
            if aa <= y {
                count -= 1;
                used[i][j] = true;
                for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                    let xi = i.wrapping_add(dx);
                    let yi = j.wrapping_add(dy);
                    if h <= xi || w <= yi {
                        continue;
                    }
                    if used[xi][yi] {
                        continue;
                    }
                    heap.push(Reverse((a[xi][yi], xi, yi)));
                }
            } else {
                heap.push(Reverse((aa, i, j)));
                break;
            }
        }
        println!("{count}");
    }
}
