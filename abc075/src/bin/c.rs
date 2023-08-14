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
    bridges: Vec<(usize, usize)>,
}

impl LowLink {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
            bridges: vec![],
        }
    }

    pub fn add_edges(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
    }

    pub fn execute(&mut self) -> usize {
        let mut pre = vec![!0; self.n];
        let mut low = vec![!0; self.n];
        let mut res = vec![];
        fn dfs(
            cur: usize,
            prev: usize,
            mut count: usize,
            g: &Vec<Vec<usize>>,
            pre: &mut Vec<usize>,
            low: &mut Vec<usize>,
            res: &mut Vec<(usize, usize)>,
        ) -> usize {
            count += 1;
            pre[cur] = count;
            low[cur] = pre[cur];
            for &next in &g[cur] {
                if pre[next] == !0 {
                    // 未到達
                    low[cur] = low[cur].min(dfs(next, cur, count, g, pre, low, res));
                    if low[next] == pre[next] {
                        res.push((cur, next));
                    }
                } else {
                    if prev == next {
                        continue;
                    }
                    low[cur] = low[cur].min(low[next]);
                }
            }
            low[cur]
        }
        dfs(0, !0, 0, &self.g, &mut pre, &mut low, &mut res);
        self.bridges = res;
        self.bridges.len()
    }
}
