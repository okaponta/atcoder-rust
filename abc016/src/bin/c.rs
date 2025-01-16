use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let w = WarshallFloyd::new(n, &g);
    for i in 0..n {
        println!(
            "{}",
            (0..n)
                .into_iter()
                .filter(|&j| w.distance[i][j] == 2)
                .count()
        );
    }
}

// 計算量はN^3
// 負の場合でも使用でき、任意の点の最短距離がすべて求まる
pub struct WarshallFloyd {
    distance: Vec<Vec<usize>>,
}

impl WarshallFloyd {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,i64)>> edges[i] = [(2,3), (3,-1), (To,重み)]
    pub fn new(n: usize, edges: &Vec<Vec<usize>>) -> Self {
        let mut distance = vec![vec![1 << 60; n]; n];
        for i in 0..n {
            distance[i][i] = 0;
            for &j in &edges[i] {
                distance[i][j] = 1;
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                }
            }
        }
        Self { distance }
    }
}
