// 二部マッチング
struct Bipartite {
    n: usize,
    k: usize,
    edge: Vec<Vec<usize>>,
    matching: Vec<i64>,
}

// 0...source/n+k+1...sink
impl Bipartite {
    pub fn new(n: usize, k: usize) -> Self {
        let mut edge = vec![vec![]; n + k + 2];
        // source
        for i in 0..n {
            edge[0].push(i + 1);
            edge[i + 1].push(0);
        }
        // sink
        for i in 0..k {
            edge[n + k + 1].push(n + i + 1);
            edge[n + i + 1].push(n + k + 1);
        }
        let matching = vec![-1; n + k + 2];
        Self {
            n,
            k,
            edge,
            matching,
        }
    }

    // 一方の集合のa個目と一方の集合のb個目をつなぐ(0-index)
    pub fn join_edge(&mut self, a: usize, b: usize) {
        let a = a + 1;
        let b = self.n + b + 1;
        self.edge[a].push(b);
        self.edge[b].push(a);
    }

    pub fn max_matching(&mut self) -> i64 {
        let mut res = 0;
        for i in 0..self.n + self.k + 2 {
            if self.matching[i] < 0 {
                if self.dfs(i, &mut vec![false; self.n + self.k + 2]) {
                    res += 1;
                }
            }
        }
        res
    }

    pub fn dfs(&mut self, cur: usize, used: &mut Vec<bool>) -> bool {
        used[cur] = true;
        for v in 0..self.edge[cur].len() {
            let u = self.edge[cur][v];
            let w = self.matching[v];
            if w < 0 || !used[w as usize] && self.dfs(w as usize, used) {
                self.matching[v] = u as i64;
                self.matching[u] = v as i64;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ford_fulkerson() {
        let n = 5;
        let k = 4;
        let st = vec![(1, 1), (2, 2), (3, 3), (3, 4), (4, 2), (5, 2)];

        let mut b = Bipartite::new(n, k);
        for (s, t) in st {
            b.join_edge(s, t);
        }
        assert_eq!(b.max_matching(), 3);
    }
}
