use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,
       m:usize,
       ab:[(Usize1,Usize1);m],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut used = vec![false; n];
    let mut ans = 1;
    let mut color = vec![0; n];
    for i in 0..n {
        if used[i] {
            continue;
        }
        let mut path = vec![];
        dfs1(i, &edges, &mut used, &mut path);
        ans *= dfs2(0, &path, &mut color, &edges);
    }
    println!("{}", ans);
}

fn dfs1(i: usize, edges: &Vec<Vec<usize>>, used: &mut Vec<bool>, path: &mut Vec<usize>) {
    if used[i] {
        return;
    }
    used[i] = true;
    path.push(i);

    for &u in &edges[i] {
        if used[u] {
            continue;
        }
        dfs1(u, &edges, used, path);
    }
}

fn dfs2(depth: usize, path: &Vec<usize>, color: &mut Vec<i32>, edges: &Vec<Vec<usize>>) -> usize {
    if depth == path.len() {
        // 最後まで塗り分け
        return 1;
    }
    let mut res = 0;
    let cur = path[depth];

    // 3色でループ
    for &c in [1, 2, 4].iter() {
        let mut same_col = false;
        for &next in &edges[cur] {
            if c == color[next] {
                same_col = true;
            }
        }
        if same_col {
            continue;
        }
        color[cur] = c;
        res += dfs2(depth + 1, path, color, edges);
        color[cur] = 0;
    }
    res
}
