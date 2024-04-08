use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        a: [i64; n],
    }
    let mut c = vec![];
    for _ in 0..n {
        input! {
            k:usize,
            ci:[usize;k],
        }
        c.push(ci);
    }
    let mut f = FordFulkerson::new(n + 2);

    let s = 0;
    let t = n + 1;

    let mut ans = 0;

    for i in 0..n {
        ans += a[i];
        // 訪れる罰金W
        f.add_edges(s, i + 1, w);
        // 訪れない罰金a[i]
        f.add_edges(i + 1, t, a[i]);
        for &j in &c[i] {
            // 鍵がなければ到達不可
            f.add_edges(i + 1, j, 1 << 60);
        }
    }
    ans -= f.max_flow();
    println!("{}", ans);
}

struct FordFulkerson {
    n: usize,
    edge: Vec<Vec<(usize, i64, usize)>>,
    flow: i64,
}

impl FordFulkerson {
    pub fn new(n: usize) -> Self {
        let flow = 0;
        let edge = vec![vec![]; n];
        Self { n, edge, flow }
    }

    pub fn add_edges(&mut self, from: usize, to: usize, cap: i64) {
        let idx = self.edge[from].len();
        let rev_idx = self.edge[to].len();
        self.edge[from].push((to, cap, rev_idx));
        self.edge[to].push((from, 0, idx));
    }

    pub fn max_flow(&mut self) -> i64 {
        loop {
            if self.next_flow() == 0 {
                return self.flow;
            }
        }
    }

    pub fn next_flow(&mut self) -> i64 {
        let f = self.dfs(0, self.n - 1, 1 << 60, &mut vec![false; self.n]);
        self.flow += f;
        f
    }

    pub fn dfs(&mut self, from: usize, to: usize, flow: i64, used: &mut Vec<bool>) -> i64 {
        if from == to {
            return flow;
        }
        used[from] = true;
        for i in 0..self.edge[from].len() {
            let (next, cap, rev) = self.edge[from][i];
            if !used[next] && cap > 0 {
                let d = self.dfs(next, to, flow.min(cap), used);
                if d > 0 {
                    self.edge[from][i].1 -= d;
                    self.edge[next][rev].1 += d;
                    return d;
                }
            }
        }
        0
    }
}
