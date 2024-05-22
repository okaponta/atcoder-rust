use proconio::{input, marker::Usize1};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        c:[char;n],
        ab:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    println!("{}", dfs(0, 0, &g, &c).2);
}

fn dfs(cur: usize, prev: usize, g: &Vec<Vec<usize>>, c: &Vec<char>) -> (usize, usize, usize) {
    let mut res = if c[cur] == 'a' { (1, 0, 1) } else { (0, 1, 1) };
    for &nxt in &g[cur] {
        if nxt == prev {
            continue;
        }
        let ret = dfs(nxt, cur, g, c);
        if c[cur] == 'a' {
            res.0 = res.0 * (ret.0 + ret.2) % MOD;
        } else {
            res.1 = res.1 * (ret.1 + ret.2) % MOD;
        }
        res.2 = res.2 * (ret.0 + ret.1 + 2 * ret.2) % MOD;
    }
    res.2 = (res.2 + MOD - if c[cur] == 'a' { res.0 } else { res.1 }) % MOD;
    res
}
