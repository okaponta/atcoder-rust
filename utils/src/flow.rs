use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

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

// Dinic法。上記より高速。
struct Dinic {
    n: usize,
    edge: Vec<Vec<(usize, i64, usize)>>,
    level: Vec<i64>,
    iter: Vec<usize>,
    flow: i64,
}

impl Dinic {
    pub fn new(n: usize) -> Self {
        let flow = 0;
        let edge = vec![vec![]; n];
        let level = vec![];
        let iter = vec![];
        Self {
            n,
            edge,
            level,
            iter,
            flow,
        }
    }

    pub fn add_edges(&mut self, from: usize, to: usize, cap: i64) {
        let idx = self.edge[from].len();
        let rev_idx = self.edge[to].len();
        self.edge[from].push((to, cap, rev_idx));
        self.edge[to].push((from, 0, idx));
    }

    pub fn max_flow(&mut self) -> i64 {
        loop {
            self.bfs();
            if self.level[self.n - 1] < 0 {
                return self.flow;
            }
            self.iter = vec![0; self.n];
            loop {
                if self.next_flow() == 0 {
                    break;
                }
            }
        }
    }

    pub fn bfs(&mut self) {
        self.level = vec![-1; self.n];
        let mut q = VecDeque::new();
        self.level[0] = 0;
        q.push_back(0);
        while let Some(v) = q.pop_front() {
            for i in 0..self.edge[v].len() {
                let (next, cap, _) = self.edge[v][i];
                if cap > 0 && self.level[next] < 0 {
                    self.level[next] = self.level[v] + 1;
                    q.push_back(next);
                }
            }
        }
    }

    pub fn next_flow(&mut self) -> i64 {
        let f = self.dfs(0, self.n - 1, 1 << 60);
        self.flow += f;
        f
    }

    pub fn dfs(&mut self, from: usize, to: usize, flow: i64) -> i64 {
        if from == to {
            return flow;
        }
        while self.iter[from] < self.edge[from].len() {
            let i = self.iter[from];
            let (next, cap, rev) = self.edge[from][i];
            if cap > 0 && self.level[from] < self.level[next] {
                let d = self.dfs(next, to, flow.min(cap));
                if d > 0 {
                    self.edge[from][i].1 -= d;
                    self.edge[next][rev].1 += d;
                    return d;
                }
            }
            self.iter[from] += 1;
        }
        0
    }
}

// 最小費用流
struct MinCostFlow {
    n: usize,
    edge: Vec<Vec<(usize, i64, i64, usize)>>,
    h: Vec<i64>,
    dist: Vec<i64>,
    prevv: Vec<usize>,
    preve: Vec<usize>,
}

impl MinCostFlow {
    pub fn new(n: usize) -> Self {
        let edge = vec![vec![]; n];
        let h = vec![0; n];
        let prevv = vec![0; n];
        let preve = vec![0; n];
        let dist = vec![1 << 60; n];
        Self {
            n,
            edge,
            h,
            dist,
            prevv,
            preve,
        }
    }

    pub fn add_edges(&mut self, from: usize, to: usize, cap: i64, cost: i64) {
        let idx = self.edge[from].len();
        let rev_idx = self.edge[to].len();
        self.edge[from].push((to, cap, cost, rev_idx));
        self.edge[to].push((from, 0, -cost, idx));
    }

    pub fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i64) -> i64 {
        let mut res = 0;
        let mut heap = BinaryHeap::new();
        while f > 0 {
            self.dist = vec![1 << 60; self.n];
            self.dist[s] = 0;
            heap.push((Reverse(0), s));
            while let Some((Reverse(d), v)) = heap.pop() {
                if self.dist[v] < d {
                    continue;
                }
                for i in 0..self.edge[v].len() {
                    let (to, cap, cost, _) = self.edge[v][i];
                    if cap > 0 && self.dist[to] > self.dist[v] + cost + self.h[v] - self.h[to] {
                        self.dist[to] = self.dist[v] + cost + self.h[v] - self.h[to];
                        self.prevv[to] = v;
                        self.preve[to] = i;
                        heap.push((Reverse(self.dist[to]), to));
                    }
                }
            }
            // これ以上流せない
            if self.dist[t] == 1 << 60 {
                return -1;
            }
            for v in 0..self.n {
                self.h[v] += self.dist[v];
            }

            // 最短経路に流す
            let mut d = f;
            let mut v = t;
            while v != s {
                // 流せるキャパシティを計算
                d = d.min(self.edge[self.prevv[v]][self.preve[v]].1);
                v = self.prevv[v];
            }
            f -= d;
            res += d * self.h[t];
            let mut v = t;
            while v != s {
                let rev = self.edge[self.prevv[v]][self.preve[v]].3;
                self.edge[self.prevv[v]][self.preve[v]].1 -= d;
                self.edge[v][rev].1 += d;
                v = self.prevv[v];
            }
            // これ以上流せない
            if self.dist[t] == 1 << 60 {
                return -1;
            }
            for v in 0..self.n {
                self.h[v] += self.dist[v];
            }

            // 最短経路に流す
            let mut d = f;
            let mut v = t;
            while v != s {
                // 流せるキャパシティを計算
                d = d.min(self.edge[self.prevv[v]][self.preve[v]].1);
                v = self.prevv[v];
            }
            f -= d;
            res += d * self.h[t];
            let mut v = t;
            while v != s {
                let rev = self.edge[self.prevv[v]][self.preve[v]].3;
                self.edge[self.prevv[v]][self.preve[v]].1 -= d;
                self.edge[v][rev].1 += d;
                v = self.prevv[v];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ford_fulkerson() {
        let n = 5;
        let stc = vec![
            (0, 1, 10),
            (0, 2, 2),
            (1, 2, 6),
            (1, 3, 6),
            (2, 4, 5),
            (3, 2, 3),
            (3, 4, 8),
        ];
        let mut ff = FordFulkerson::new(n);
        for (s, t, c) in stc {
            ff.add_edges(s, t, c);
        }
        assert_eq!(ff.max_flow(), 11);
    }

    #[test]
    fn test_dinic() {
        let n = 5;
        let stc = vec![
            (0, 1, 10),
            (0, 2, 2),
            (1, 2, 6),
            (1, 3, 6),
            (2, 4, 5),
            (3, 2, 3),
            (3, 4, 8),
        ];
        let mut ff = Dinic::new(n);
        for (s, t, c) in stc {
            ff.add_edges(s, t, c);
        }
        assert_eq!(ff.max_flow(), 11);
    }

    #[test]
    fn test_min_cost_flow() {
        let n = 5;
        let stcd = vec![
            (0, 1, 10, 2),
            (0, 2, 2, 4),
            (1, 2, 6, 6),
            (1, 3, 6, 2),
            (2, 4, 5, 2),
            (3, 2, 3, 3),
            (3, 4, 8, 6),
        ];
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 0), 0);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 1), 6);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 2), 12);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 3), 21);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 4), 30);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 5), 39);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 6), 49);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 7), 59);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 8), 69);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 9), 80);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 10), 91);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 11), 102);
        assert_eq!(setupmcf(n, &stcd).min_cost_flow(0, n - 1, 12), -1);
    }

    fn setupmcf(n: usize, stcd: &Vec<(usize, usize, i64, i64)>) -> MinCostFlow {
        let mut mcf = MinCostFlow::new(n);
        for &(s, t, c, d) in stcd {
            mcf.add_edges(s, t, c, d);
        }
        mcf
    }
}
