use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
        q:usize,
        uk:[(Usize1,usize);q],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }
    // l,rのどちらかが最も遠い点になるのでこの2点のみを検証すればOK
    let (l, r, _) = tree_diameter(&edges);
    let mut llca = LCA::new(n);
    llca.init(&edges, l);
    let mut rlca = LCA::new(n);
    rlca.init(&edges, r);

    for (u, k) in uk {
        let lca;
        if llca.get_dist(u, l) >= k {
            lca = &llca;
        } else if rlca.get_dist(u, r) >= k {
            lca = &rlca;
        } else {
            // どっちの直径の距離にも満たないので不可能
            println!("-1");
            continue;
        }
        let mut ans = u;
        // 2^n単位で親へ移動していく、lかrが最も遠い点なので、親にたどるだけでOK
        for i in (0..lca.k).rev() {
            if k >> i & 1 == 1 {
                ans = lca.parent[i][ans];
            }
        }
        println!("{}", ans + 1);
    }
}

// 直径の片方ともう片方と直径
fn tree_diameter(edges: &Vec<Vec<usize>>) -> (usize, usize, usize) {
    let l = tree_diameter_dfs(edges, 0, !0);
    let r = tree_diameter_dfs(edges, l.1, !0);
    (l.1, r.1, r.0)
}

// (距離, to)の最大値を返却する
fn tree_diameter_dfs(edges: &Vec<Vec<usize>>, cur: usize, parent: usize) -> (usize, usize) {
    let mut ret = (0, cur);
    for &next in &edges[cur] {
        if next == parent {
            continue;
        }
        let mut hoge = tree_diameter_dfs(edges, next, cur);
        hoge.0 += 1;
        ret = ret.max(hoge);
    }
    ret
}

// LCA (Lowest Common Ancestor)
// 実装スキル不足でnewしたあとにinitをしなきゃいけない構造になってしまった・・・
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
