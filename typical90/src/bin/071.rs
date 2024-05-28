use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    let mut ind = vec![0; n];
    for (a, b) in ab {
        g[a].push(b);
        ind[b] += 1;
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if ind[i] == 0 {
            q.push_back(i);
        }
    }

    let mut tmp = vec![0usize; n];
    let mut ans = vec![];
    dfs(&g, &mut q, &mut ind, &mut tmp, 0, k, &mut ans);

    if ans.len() < k {
        println!("-1");
    } else {
        for ans in ans {
            println!("{}", ans);
        }
    }
}

fn dfs(
    g: &Vec<Vec<usize>>,
    q: &mut VecDeque<usize>,
    ind: &mut Vec<i32>,
    tmp: &mut Vec<usize>,
    i: usize,
    k: usize,
    ans: &mut Vec<String>,
) {
    if i == tmp.len() {
        ans.push(tmp.iter().join(" "));
        return;
    }

    for _ in 0..q.len() {
        let cur = q.pop_front().unwrap();
        tmp[i] = cur + 1;
        for &u in &g[cur] {
            ind[u] -= 1;
            if ind[u] == 0 {
                q.push_back(u);
            }
        }

        dfs(g, q, ind, tmp, i + 1, k, ans);

        if ans.len() == 0 || ans.len() == k {
            return;
        }

        for &next in &g[cur] {
            if ind[next] == 0 {
                q.pop_back();
            }
            ind[next] += 1;
        }
        tmp[i] = 0;
        q.push_back(cur);
    }
}
