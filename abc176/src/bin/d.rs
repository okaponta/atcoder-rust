use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

const INF: usize = 1 << 60;

fn main() {
    input! {
        h:usize,w:usize,
        c:(Usize1,Usize1),
        d:(Usize1,Usize1),
        s:[Chars;h],
    }
    let mut count = vec![vec![INF; w + 4]; h + 4];
    let mut done = vec![vec![false; w + 4]; h + 4];
    let mut is_path = vec![vec![false; w + 4]; h + 4];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                is_path[i + 2][j + 2] = true;
            }
        }
    }
    count[c.0 + 2][c.1 + 2] = 0;
    let mut warpq = vec![];
    let mut q = VecDeque::new();
    q.push_back((c.0 + 2, c.1 + 2, 0));
    warpq.push((c.0 + 2, c.1 + 2, 0));

    while !q.is_empty() && !&warpq.is_empty() {
        while let Some(walk) = q.pop_back() {
            for i in 0..3 {
                for j in 0..3 {
                    if (i == 0 && (j == 0 || j == 2)) || (i == 2 && (j == 0 || j == 2)) {
                        continue;
                    }
                    if is_path[walk.0 - 1 + i][walk.1 - 1 + j]
                        && !done[walk.0 - 1 + i][walk.1 - 1 + j]
                        && count[walk.0 - 1 + i][walk.1 - 1 + j] > walk.2
                    {
                        count[walk.0 - 1 + i][walk.1 - 1 + j] = walk.2;
                        done[walk.0 - 1 + i][walk.1 - 1 + j] = true;
                        q.push_back((walk.0 - 1 + i, walk.1 - 1 + j, walk.2));
                        warpq.push((walk.0 - 1 + i, walk.1 - 1 + j, walk.2));
                    }
                }
            }
        }
        let mut next = vec![];
        for warp in warpq {
            for i in 0..5 {
                for j in 0..5 {
                    if is_path[warp.0 - 2 + i][warp.1 - 2 + j]
                        && !done[warp.0 - 2 + i][warp.1 - 2 + j]
                        && count[warp.0 - 2 + i][warp.1 - 2 + j] > warp.2 + 1
                    {
                        count[warp.0 - 2 + i][warp.1 - 2 + j] = warp.2 + 1;
                        done[warp.0 - 2 + i][warp.1 - 2 + j] = true;
                        q.push_back((warp.0 - 2 + i, warp.1 - 2 + j, warp.2 + 1));
                        next.push((warp.0 - 2 + i, warp.1 - 2 + j, warp.2 + 1));
                    }
                }
            }
        }
        warpq = next;
    }
    let ans = count[d.0 + 2][d.1 + 2];
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
