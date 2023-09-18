use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        abxy:[(Usize1,Usize1,i64,i64);m],
    }
    let mut g = vec![vec![]; n];
    for (a, b, x, y) in abxy {
        g[a].push((b, x, y));
        g[b].push((a, -x, -y));
    }
    let mut used = vec![false; n];
    let mut ans = vec![(0, 0); n];
    used[0] = true;
    dfs(0, 0, &mut used, &g, &mut ans);
    for i in 0..n {
        if used[i] {
            println!("{} {}", ans[i].0, ans[i].1);
        } else {
            println!("undecidable");
        }
    }
}

fn dfs(
    cur: usize,
    prev: usize,
    used: &mut Vec<bool>,
    g: &Vec<Vec<(usize, i64, i64)>>,
    ans: &mut Vec<(i64, i64)>,
) {
    for &(next, dx, dy) in &g[cur] {
        if next == prev {
            continue;
        }
        if used[next] {
            continue;
        }
        used[next] = true;
        ans[next] = (ans[cur].0 + dx, ans[cur].1 + dy);
        dfs(next, cur, used, g, ans);
    }
}
