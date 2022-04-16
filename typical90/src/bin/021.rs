use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut edges = vec![vec![]; n];
    let mut rev_edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        rev_edges[b].push(a);
    }
    let mut scc = SCC::new(n);
    scc.execute(edges, rev_edges);
    let ans = scc.sizes.iter().fold(0, |acc, x| acc + x * (x - 1) / 2);
    println!("{}", ans);
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
