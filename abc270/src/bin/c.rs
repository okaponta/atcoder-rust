use itertools::{self, Itertools};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        x:Usize1,
        y:Usize1,
        uv:[(Usize1,Usize1);n-1],
    }
    let mut edges = vec![vec![]; n];
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut lca = LCA::new(n);
    lca.init(&edges, 0);
    let lcaxy = lca.query(x, y);
    let mut x_path = vec![x];
    let mut cur = x;
    while cur != lcaxy {
        cur = lca.parent[0][cur];
        x_path.push(cur);
    }
    let mut y_path = vec![y];
    let mut cur = y;
    while cur != lcaxy {
        cur = lca.parent[0][cur];
        y_path.push(cur);
    }
    let mut ans = vec![];
    for i in x_path {
        ans.push(i + 1);
    }
    for i in (0..y_path.len() - 1).rev() {
        ans.push(y_path[i] + 1);
    }
    println!("{}", ans.iter().join(" "));
}

pub struct LCA {
    n: usize,
    k: usize,
    // とある頂点の1,2,4...個先の頂点を保持
    parent: Vec<Vec<usize>>,
    dist: Vec<usize>,
}

impl LCA {
    pub fn new(n: usize) -> Self {
        const INF: usize = 1 << 60;
        let mut k = 1;
        while 1 << k < n {
            k += 1;
        }
        let parent = vec![vec![INF; n]; k];
        let dist = vec![INF; n];
        Self { n, k, parent, dist }
    }

    pub fn init(&mut self, edges: &Vec<Vec<usize>>, init: usize) {
        const INF: usize = 1 << 60;
        self.dfs(init, init, edges);
        for i in 0..self.k - 1 {
            for j in 0..self.n {
                if self.parent[i][j] == INF {
                    self.parent[i + 1][j] = INF;
                } else {
                    self.parent[i + 1][j] = self.parent[i][self.parent[i][j]];
                }
            }
        }
    }

    // uとvのLCAを求める
    pub fn query(&self, mut u: usize, mut v: usize) -> usize {
        // uを深い方として計算する
        if self.dist[u] < self.dist[v] {
            std::mem::swap(&mut u, &mut v);
        }
        // LCAまでの距離を同じにする
        for i in 0..self.k {
            if (self.dist[u] - self.dist[v]) >> i & 1 == 1 {
                u = self.parent[i][u];
            }
        }
        if u == v {
            return u;
        }
        // 一定以上の祖先は同じはずなので、上のbitから検証して揃えていく
        for i in (0..self.k).rev() {
            if self.parent[i][u] != self.parent[i][v] {
                u = self.parent[i][u];
                v = self.parent[i][v];
            }
        }
        return self.parent[0][u];
    }

    // 2点間の距離
    pub fn get_dist(&self, u: usize, v: usize) -> usize {
        self.dist[u] + self.dist[v] - 2 * self.dist[self.query(u, v)]
    }

    // 2点間のパス上に点pが存在するか判定
    pub fn is_on_path(&self, u: usize, v: usize, p: usize) -> bool {
        self.get_dist(u, p) + self.get_dist(p, v) == self.get_dist(u, v)
    }

    // 根からの距離を求める
    fn dfs(&mut self, prev: usize, cur: usize, edges: &Vec<Vec<usize>>) {
        for &next in &edges[cur] {
            if next == prev {
                continue;
            }
            self.dist[next] = self.dist[cur] + 1;
            self.parent[0][next] = cur;
            self.dfs(cur, next, edges);
        }
    }
}
