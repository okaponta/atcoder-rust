use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
        q:usize,
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut lca = LCA::new(n);
    lca.init(&g, 0);

    let mut et: Vec<usize> = vec![n; 2 * n - 1];
    let mut depth: Vec<usize> = vec![0; 2 * n - 1];
    let mut d: Vec<usize> = vec![0; n];
    let mut idx: Vec<(usize, usize)> = vec![(0, 0); n];
    dfs(0, n, &mut 0, &g, &mut et, &mut d, &mut depth, &mut idx);

    let mut ans = vec![];
    for _ in 0..q {
        input! {
            k: usize,
            v: [Usize1; k],
        }
        let mut v = v.into_iter().sorted_by_key(|v| idx[*v].0).collect_vec();
        v.push(*v.first().unwrap());

        let dist = v
            .windows(2)
            .map(|v| lca.get_dist(v[0], v[1]))
            .sum::<usize>();
        ans.push(dist / 2);
    }
    println!("{}", ans.iter().join("\n"));
}

// オイラーツアー
// let mut et: Vec<usize> = vec![n; 2 * n - 1]; // ノードを巡る順番(行きと帰りが記録される)
// let mut depth: Vec<usize> = vec![0; 2 * n - 1]; // 深さ
// let mut d: Vec<usize> = vec![0; n]; // 順番(1回だけ)
// let mut idx: Vec<(usize, usize)> = vec![(0, 0); n]; // 行きと帰りのインデックス
fn dfs(
    i: usize,
    par: usize,
    k: &mut usize,
    g: &Vec<Vec<usize>>,
    et: &mut Vec<usize>,
    d: &mut Vec<usize>,
    depth: &mut Vec<usize>,
    idx: &mut Vec<(usize, usize)>,
) {
    et[*k] = i;
    idx[i].0 = *k;
    *k += 1;
    for &ni in &g[i] {
        if ni != par {
            d[ni] = d[i] + 1;
            depth[*k] = d[ni];
            dfs(ni, i, k, g, et, d, depth, idx);
            et[*k] = i;
            depth[*k] = d[i];
            idx[i].1 = *k;
            *k += 1;
        }
    }
}

// LCA (Lowest Common Ancestor)
// 実装スキル不足でnewしたあとにinitをしなきゃいけない構造になってしまった・・・
pub struct LCA {
    n: usize,
    k: usize,
    // とある頂点の1,2,4...個先の頂点を保持
    // parent[0][i]...iの頂点の1つ祖先
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
