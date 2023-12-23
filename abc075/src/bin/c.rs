use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut l = LowLink::new(n);
    for (a, b) in ab {
        l.add_edges(a, b);
    }
    println!("{}", l.execute());
}

pub struct LowLink {
    n: usize,
    g: Vec<Vec<usize>>,
    articulation_points: Vec<usize>,
    bridges: Vec<(usize, usize)>,
}

impl LowLink {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
            articulation_points: vec![],
            bridges: vec![],
        }
    }

    pub fn add_edges(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
    }

    pub fn execute(&mut self) -> usize {
        let mut ord = vec![!0; self.n];
        let mut low = vec![!0; self.n];
        let mut bridges = vec![];
        let mut articulation = vec![];
        fn dfs(
            cur: usize,
            prev: usize,
            mut count: usize,
            g: &Vec<Vec<usize>>,
            used: &mut Vec<bool>,
            ord: &mut Vec<usize>,
            low: &mut Vec<usize>,
            articulation: &mut Vec<usize>,
            bridges: &mut Vec<(usize, usize)>,
        ) -> usize {
            used[cur] = true;
            count += 1;
            ord[cur] = count;
            low[cur] = ord[cur];
            let mut is_articulation = false;
            let mut childs = 0;
            for &next in &g[cur] {
                if !used[next] {
                    childs += 1;
                    count = dfs(next, cur, count, g, used, ord, low, articulation, bridges);
                    low[cur] = low[cur].min(low[next]);
                    if prev != !0 && ord[cur] <= low[next] {
                        is_articulation = true;
                    }
                    if ord[cur] < low[next] {
                        bridges.push((cur, next));
                    }
                } else if next != prev {
                    low[cur] = low[cur].min(ord[next]);
                }
            }
            if prev == !0 && 2 <= childs {
                is_articulation = true;
            }
            if is_articulation {
                articulation.push(cur);
            }
            count
        }
        let mut used = vec![false; self.n];
        for i in 0..self.n {
            if !used[i] {
                dfs(
                    i,
                    !0,
                    0,
                    &self.g,
                    &mut used,
                    &mut ord,
                    &mut low,
                    &mut articulation,
                    &mut bridges,
                );
            }
        }
        self.bridges = bridges;
        self.articulation_points = articulation;
        self.bridges.len()
    }
}
