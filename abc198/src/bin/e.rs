use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,
       c:[usize;n],
       ab:[(Usize1,Usize1);n-1],
    }
    let mut edge = vec![vec![]; n];
    for (a, b) in ab {
        edge[a].push(b);
        edge[b].push(a);
    }
    let mut color = vec![0; 100001];
    let mut ans = vec![false; n];
    dfs(&c, &edge, 0, 0, &mut color, &mut ans);
    for i in 0..n {
        if ans[i] {
            println!("{}", i + 1);
        }
    }
}

fn dfs(
    c: &Vec<usize>,
    edge: &Vec<Vec<usize>>,
    cur: usize,
    prev: usize,
    count: &mut Vec<usize>,
    ans: &mut Vec<bool>,
) {
    count[c[cur]] += 1;
    if count[c[cur]] == 1 {
        ans[cur] = true;
    }
    for &v in &edge[cur] {
        if v == prev {
            continue;
        }
        dfs(c, edge, v, cur, count, ans);
    }
    count[c[cur]] -= 1;
}
