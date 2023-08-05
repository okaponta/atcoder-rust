use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[b].push(a);
    }
    let mut goal = vec![n; n];
    for i in 0..n {
        dfs(n, i, &mut goal, &g);
    }
    goal.sort();
    goal.dedup();
    if goal.len() == 1 {
        println!("{}", goal[0] + 1);
    } else {
        println!("-1");
    }
}

fn dfs(n: usize, cur: usize, goal: &mut Vec<usize>, g: &Vec<Vec<usize>>) {
    if goal[cur] != n {
        return;
    }
    if g[cur].len() == 0 {
        goal[cur] = cur;
        return;
    }
    for &next in &g[cur] {
        dfs(n, next, goal, g);
        goal[cur] = goal[next];
    }
}
