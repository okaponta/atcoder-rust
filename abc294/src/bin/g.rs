use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut uvw:[(Usize1,Usize1,i64);n-1],
        q:usize,
    }
    let mut edges = vec![vec![]; n];
    for i in 0..n - 1 {
        edges[uvw[i].0].push(uvw[i].1);
        edges[uvw[i].1].push(uvw[i].0);
    }
    let mut lca = LCA::new(n);
    lca.init(&edges, 0);
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[uvw[i].0].push((uvw[i].1, uvw[i].2, i));
        g[uvw[i].1].push((uvw[i].0, uvw[i].2, i));
    }
    let mut fw = FenwickTree::new(2 * n + 2);
    let mut vertex = vec![0; n];
    let mut edges = vec![(0, 0); n];
    dfs(0, 0, 1, &mut fw, &g, &mut vertex, &mut edges);
    for _ in 0..q {
        input! {q: u8}
        if q == 1 {
            input! {i: Usize1, w :i64}
            fw.add(edges[i].0, -uvw[i].2 + w);
            fw.add(edges[i].1, uvw[i].2 - w);
            uvw[i].2 = w;
        } else {
            input! {u:Usize1,v:Usize1}
            let l = lca.query(u, v);
            println!(
                "{}",
                fw.sum(vertex[u]) + fw.sum(vertex[v]) - 2 * fw.sum(vertex[l])
            )
        }
    }
}

fn dfs(
    cur: usize,
    prev: usize,
    mut idx: usize,
    fw: &mut FenwickTree,
    g: &Vec<Vec<(usize, i64, usize)>>,
    vertex: &mut Vec<usize>,
    edges: &mut Vec<(usize, usize)>,
) -> usize {
    for &(next, cost, i) in &g[cur] {
        if next == prev {
            continue;
        }
        fw.add(idx, cost);
        edges[i].0 = idx;
        vertex[next] = idx;
        idx = dfs(next, cur, idx + 1, fw, g, vertex, edges);
        idx += 1;
        fw.add(idx, -cost);
        edges[i].1 = idx;
    }
    idx
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

pub struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    // a1~anの配列を作成
    pub fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    // aiにvを加算する
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

    // a1+a2+...aiを計算する
    pub fn sum(&self, i: usize) -> i64 {
        assert!(i < self.len);
        let mut i = i as i64;
        let mut sum = 0;
        while i > 0 {
            sum += self.data[i as usize];
            i -= i & -i;
        }
        sum
    }

    // ai+...+ajを計算する
    pub fn range(&self, i: usize, j: usize) -> i64 {
        assert!(i <= j);
        assert!(j < self.len);
        self.sum(j) - self.sum(i - 1)
    }

    // 和がs以下の位置を返却
    pub fn lower(&self, s: i64) -> usize {
        let mut lower = 0;
        let mut upper = self.len;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if self.sum(med) <= s {
                lower = med;
            } else {
                upper = med;
            }
        }
        lower
    }
}
