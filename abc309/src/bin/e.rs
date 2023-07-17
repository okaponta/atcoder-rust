use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        p:[Usize1;n-1],
        xy:[(Usize1,usize);m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[p[i]].push(i + 1);
    }
    let mut ins = vec![0; n];
    for (x, y) in xy {
        ins[x] = ins[x].max(y + 1);
    }
    println!("{}", dfs(0, 0, &g, &ins));
}

fn dfs(cur: usize, d: usize, g: &Vec<Vec<usize>>, ins: &Vec<usize>) -> usize {
    let dd = d.max(ins[cur]);
    let mut res = 0;
    if 0 < dd {
        res += 1;
    }
    for &next in &g[cur] {
        res += dfs(next, dd.saturating_sub(1), g, ins);
    }
    res
}
