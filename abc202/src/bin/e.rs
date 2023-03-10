use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
       n:usize,
       p:[Usize1;n-1],
       q:usize,
       ud:[(Usize1,usize);q],
    }
    let mut child = vec![vec![]; n];
    for i in 1..n {
        child[p[i - 1]].push(i);
    }
    let mut qs = vec![vec![]; n];
    for i in 0..q {
        qs[ud[i].0].push((ud[i].1, i));
    }
    let mut ans = vec![(0, 0); q];
    let mut depth = vec![0; n];
    dfs(0, 0, &child, &mut ans, &mut depth, &qs);
    for i in 0..q {
        println!("{}", ans[i].1 - ans[i].0);
    }
}

fn dfs(
    cur: usize,
    dep: usize,
    child: &Vec<Vec<usize>>,
    ans: &mut Vec<(usize, usize)>,
    depth: &mut Vec<usize>,
    qs: &Vec<Vec<(usize, usize)>>,
) {
    for &(d, i) in &qs[cur] {
        ans[i].0 = depth[d];
    }
    depth[dep] += 1;
    for &next in &child[cur] {
        dfs(next, dep + 1, child, ans, depth, qs);
    }
    for &(d, i) in &qs[cur] {
        ans[i].1 = depth[d];
    }
}
