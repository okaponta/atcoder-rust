use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use itertools::{iproduct, Itertools};

const INF: usize = 1 << 60;

// 計算量は(E+V)logV
struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        let mut parent = vec![INF; n];
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((Reverse(0), i));
            }
            heap.push((Reverse(INF), i));
        }
        while let Some((Reverse(d), target)) = heap.pop() {
            if distance[target] < d {
                continue;
            }
            distance[target] = d;
            for &(next, cost) in &edge[target] {
                if distance[next] > d + cost {
                    distance[next] = d + cost;
                    heap.push((Reverse(distance[next]), next));
                    parent[next] = target;
                }
            }
        }
        Self { distance, parent }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }

    pub fn get_path(&self, target: usize) -> Vec<usize> {
        let mut current = target;
        let mut res = vec![current];
        while self.parent[current] != INF as usize {
            let next = self.parent[current];
            res.push(next);
            current = next;
        }
        res.reverse();
        res
    }
}

// 計算量はE×V
struct BellmanFord {
    distance: Vec<i64>,
    has_neg_loop: bool,
}

impl BellmanFord {
    // n:usize 頂点の数
    // edges: Vec<(usize,usize,i64)> edges[i] = [(0,2,3), (1,3,-1), (From,To,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: Vec<(usize, usize, i64)>, init: usize) -> Self {
        let mut distance = vec![1 << 60; n];
        distance[init] = 0;
        let mut has_neg_loop = false;

        for i in 0..n {
            for edge in &edges {
                let from = edge.0;
                let to = edge.1;
                let cost = edge.2;
                if distance[to] > distance[from] + cost {
                    distance[to] = distance[from] + cost;
                    if i == n - 1 {
                        has_neg_loop = true;
                        break;
                    }
                }
            }
        }
        Self {
            distance,
            has_neg_loop,
        }
    }

    pub fn distance(&self, target: usize) -> i64 {
        self.distance[target]
    }
}

// 計算量はN^3
// 負の場合でも使用でき、任意の点の最短距離がすべて求まる
struct WarshallFloyd {
    distance: Vec<Vec<i64>>,
}

impl WarshallFloyd {
    // n:usize 頂点の数
    // edges: Vec<(usize,usize,i64)> edges[i] = [(0,2,3), (1,3,-1), (From,To,重み)]
    pub fn new(n: usize, edges: Vec<(usize, usize, i64)>) -> Self {
        let mut distance = vec![vec![1 << 60; n]; n];

        for &(a, b, c) in &edges {
            distance[a][b] = c;
            distance[b][a] = c;
        }

        for (i, j, k) in iproduct!(0..n, 0..n, 0..n) {
            distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
        }
        Self { distance }
    }
}

// SCC(強連結成分分解)
// n もとの頂点数
// sizes 強連結成分をまとめたときのサイズ
// new_num もとの頂点->まとめたあとの頂点のマッピング
// new_edges まとめたあとの辺(トポロジカルソート済)
struct SCC {
    n: usize,
    sizes: Vec<usize>,
    new_num: Vec<usize>,
    new_edges: Vec<Vec<usize>>,
}

impl SCC {
    pub fn new(n: usize) -> Self {
        let sizes = vec![];
        let new_num = vec![0; n];
        let new_edges = vec![];
        Self {
            n,
            sizes,
            new_num,
            new_edges,
        }
    }

    // edges/rev_edges もとの辺
    pub fn execute(&mut self, edges: Vec<Vec<usize>>, rev_edges: Vec<Vec<usize>>) {
        let n = self.n;
        let mut used = vec![false; n];
        // num[i] -> i番目の番号がどの頂点か(一度目のdfsの結果を記録)
        let mut num = vec![0; n];
        let mut count = 0;
        for i in 0..n {
            if !used[i] {
                count = self.dfs(i, count, &mut used, &mut num, &edges);
            }
        }
        // 初期化して二度目のdfsで使い回し
        used = vec![false; n];
        let mut count = 0;
        for i in (0..n).rev() {
            let target = num[i];
            if !used[target] {
                let size = self.rev_dfs(target, count, 0, &mut used, &rev_edges);
                self.sizes.push(size);
                count += 1;
            }
        }
        let mut new_edges = vec![BTreeSet::new(); self.sizes.len()];
        for i in 0..n {
            for &edge in &edges[i] {
                if self.new_num[i] != self.new_num[edge] {
                    new_edges[self.new_num[i]].insert(self.new_num[edge]);
                }
            }
        }
        self.new_edges = new_edges
            .iter()
            .map(|s| s.iter().map(|i| *i).collect_vec())
            .collect_vec();
        //return (sizes, new_num, v);
    }

    fn dfs(
        &mut self,
        cur: usize,
        mut count: usize,
        used: &mut Vec<bool>,
        num: &mut Vec<usize>,
        edges: &Vec<Vec<usize>>,
    ) -> usize {
        used[cur] = true;
        for &next in edges[cur].iter() {
            if !used[next] {
                count = self.dfs(next, count, used, num, edges);
            }
        }
        num[count] = cur;
        count + 1
    }

    fn rev_dfs(
        &mut self,
        cur: usize,
        count: usize,
        mut size: usize,
        used: &mut Vec<bool>,
        rev_edges: &Vec<Vec<usize>>,
    ) -> usize {
        used[cur] = true;
        for &next in rev_edges[cur].iter() {
            if !used[next] {
                size = self.rev_dfs(next, count, size, used, rev_edges);
            }
        }
        self.new_num[cur] = count;
        size + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkistra() {
        let n = 6;
        let abc = vec![
            (0, 1, 5),
            (0, 3, 9),
            (0, 4, 2),
            (1, 2, 2),
            (2, 3, 3),
            (3, 5, 2),
            (4, 5, 3),
        ];
        let mut path = vec![vec![]; n];
        for (a, b, c) in abc {
            path[a].push((b, c));
            path[b].push((a, c));
        }
        let d = Dijkstra::new(n, path, 0);

        assert_eq!(d.distance, vec![0, 5, 7, 7, 2, 5]);

        assert_eq!(d.get_path(0), vec![0]);
        assert_eq!(d.get_path(1), vec![0, 1]);
        assert_eq!(d.get_path(2), vec![0, 1, 2]);
        assert_eq!(d.get_path(3), vec![0, 4, 5, 3]);
        assert_eq!(d.get_path(4), vec![0, 4]);
        assert_eq!(d.get_path(5), vec![0, 4, 5]);
    }

    #[test]
    fn test_bellmanford() {
        let n = 6;
        let abc = vec![
            (0, 1, 5),
            (0, 3, 9),
            (0, 4, 2),
            (1, 2, 2),
            (2, 3, 3),
            (3, 5, 2),
            (4, 5, 3),
        ];
        let mut edges = vec![];
        for (a, b, c) in abc {
            edges.push((a, b, c));
            edges.push((b, a, c));
        }
        let b = BellmanFord::new(n, edges, 0);

        assert_eq!(b.distance, vec![0, 5, 7, 7, 2, 5]);
        assert_eq!(b.has_neg_loop, false);
    }

    #[test]
    fn test_scc() {
        let n = 12;
        let uv = vec![
            (0, 1),
            (1, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (4, 2),
            (4, 5),
            (5, 6),
            (6, 7),
            (6, 8),
            (6, 9),
            (7, 5),
            (8, 10),
            (9, 8),
            (9, 11),
            (10, 8),
        ];
        let mut edges = vec![vec![]; n];
        let mut rev_edges = vec![vec![]; n];
        for (u, v) in uv {
            edges[u].push(v);
            rev_edges[v].push(u);
        }
        let mut scc = SCC::new(n);
        scc.execute(edges, rev_edges);
        assert_eq!(scc.sizes, vec![1, 1, 3, 3, 1, 1, 2]);
        assert_eq!(scc.new_num, vec![0, 1, 2, 2, 2, 3, 3, 3, 6, 4, 6, 5]);
        assert_eq!(
            scc.new_edges,
            vec![
                vec![1],
                vec![2],
                vec![3],
                vec![4, 6],
                vec![5, 6],
                vec![],
                vec![]
            ]
        );
    }
}
