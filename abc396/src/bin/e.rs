use itertools::*;
use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        m:usize,
        xyz:[(Usize1,Usize1,usize);m],
    }
    let mut g = vec![vec![]; n];
    for (x, y, z) in xyz {
        g[x].push((y, z));
        g[y].push((x, z));
    }
    let mut ans = vec![0usize; n];
    let mut ng = false;
    for i in 0..32 {
        let mut hist = vec![2; n];
        for j in 0..n {
            if hist[j] != 2 {
                continue;
            }
            let mut v0 = vec![];
            let mut v1 = vec![];
            dfs(j, i, 0, &g, &mut v0, &mut v1, &mut hist, &mut ng);
            if v0.len() < v1.len() {
                for &k in &v0 {
                    ans[k] |= 1 << i;
                }
            } else {
                for &k in &v1 {
                    ans[k] |= 1 << i;
                }
            }
        }
    }
    if ng {
        println!("-1");
        return;
    }
    println!("{}", ans.iter().join(" "));
}

fn dfs(
    cur: usize,
    bit: usize,
    curbit: usize,
    g: &Vec<Vec<(usize, usize)>>,
    v0: &mut Vec<usize>,
    v1: &mut Vec<usize>,
    hist: &mut Vec<usize>,
    ng: &mut bool,
) {
    if hist[cur] != 2 {
        if hist[cur] != curbit {
            *ng = true;
        }
        return;
    }
    hist[cur] = curbit;
    if curbit == 0 {
        v0.push(cur);
    } else {
        v1.push(cur);
    }
    for &(next, cost) in &g[cur] {
        dfs(next, bit, curbit ^ ((cost >> bit) & 1), g, v0, v1, hist, ng);
    }
}
