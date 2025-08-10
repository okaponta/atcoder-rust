#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[usize;w];h],
    }
    println!(
        "{}",
        dfs(0, 0, vec![vec![false; w]; h], h, w, &a, &mut HashMap::new())
    );
}

fn dfs(
    mut i: usize,
    mut j: usize,
    cur: Vec<Vec<bool>>,
    h: usize,
    w: usize,
    a: &Vec<Vec<usize>>,
    memo: &mut HashMap<Vec<Vec<bool>>, usize>,
) -> usize {
    if let Some(val) = memo.get(&cur) {
        return *val;
    }
    let mut res = xor(&cur, a, h, w);
    if w <= j {
        j -= w;
        i += 1;
    }
    if h <= i {
        return res;
    }
    // たて
    if i < h - 1 && !cur[i][j] && !cur[i + 1][j] {
        let mut next = cur.clone();
        next[i][j] = true;
        next[i + 1][j] = true;
        res = res.max(dfs(i, j + 1, next, h, w, a, memo));
    }
    // よこ
    if j < w - 1 && !cur[i][j] && !cur[i][j + 1] {
        let mut next = cur.clone();
        next[i][j] = true;
        next[i][j + 1] = true;
        res = res.max(dfs(i, j + 2, next, h, w, a, memo));
    }
    res = res.max(dfs(i, j + 1, cur.clone(), h, w, a, memo));
    memo.insert(cur, res);
    res
}

fn xor(cur: &Vec<Vec<bool>>, a: &Vec<Vec<usize>>, h: usize, w: usize) -> usize {
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if !cur[i][j] {
                res ^= a[i][j]
            }
        }
    }
    res
}
