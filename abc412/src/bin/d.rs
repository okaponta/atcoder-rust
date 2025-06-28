#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut set = HashSet::new();
    for (a, b) in ab {
        set.insert((a, b));
        set.insert((b, a));
    }
    let mut ans = 1 << 60;
    for v in (0..n).into_iter().permutations(n) {
        ans = ans.min(dfs(0, 1, 0, n, &v, 0, 0, &set));
    }
    println!("{}", ans);
}

fn dfs(
    i: usize,
    j: usize,
    start: usize,
    n: usize,
    v: &Vec<usize>,
    mut add: usize,
    mut used: usize,
    set: &HashSet<(usize, usize)>,
) -> usize {
    if set.contains(&(v[i], v[j])) {
        used += 1;
    } else {
        add += 1;
    }
    // 最後
    if j == n - 1 {
        if set.contains(&(v[j], v[start])) {
            used += 1;
        } else {
            add += 1;
        }
        return set.len() / 2 - used + add;
    }
    let mut res = 1 << 60;
    // サイクルにする
    if j < n - 3 && start != i {
        if set.contains(&(v[j], v[start])) {
            res = res.min(dfs(i + 2, j + 2, i + 2, n, v, add, used + 1, set));
        } else {
            res = res.min(dfs(i + 2, j + 2, i + 2, n, v, add + 1, used, set));
        }
    }
    // サイクルにしない
    res = res.min(dfs(i + 1, j + 1, start, n, v, add, used, set));
    res
}
