use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

const INF: usize = usize::max_value();

fn main() {
    input! {
        n:usize,
        t:i64,
        a:[(i64,i64);n],
        b:[(i64,i64);n],
    }
    let mut amap = HashMap::new();
    for i in 0..n {
        amap.insert(a[i], i);
    }

    let dx = vec![t, t, 0, -t, -t, -t, 0, t];
    let dy = vec![0, t, t, t, 0, -t, -t, -t];

    let mut g = vec![vec![]; n];
    for i in 0..n {
        let (bx, by) = b[i];
        for j in 0..8 {
            let from = (bx + dx[j], by + dy[j]);
            if let Some(&k) = amap.get(&from) {
                g[i].push(k);
            }
        }
    }

    let mut matched = vec![INF; n];
    for i in 0..n {
        let mut visited = vec![false; n];
        if !dfs(i, &g, &mut visited, &mut matched) {
            println!("No");
            return;
        }
    }

    println!("Yes");
    let mut ans = vec![];
    for i in 0..n {
        let j = matched[i];
        let (ax, ay) = a[i];
        let (bx, by) = b[j];
        for j in 0..8 {
            if bx - ax == dx[j] && by - ay == dy[j] {
                ans.push(j + 1);
                break;
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}

fn dfs(s: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, matched: &mut Vec<usize>) -> bool {
    for &u in &g[s] {
        if visited[u] {
            continue;
        }
        visited[u] = true;

        if matched[u] == INF || dfs(matched[u], g, visited, matched) {
            matched[u] = s;
            return true;
        }
    }
    false
}
