use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        c:usize,
        d:[[usize;n];n],
    }
    let mut g1 = vec![vec![]; n];
    let mut g2 = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                g1[i].push((j, d[i][j] * a));
                g2[i].push((j, d[i][j] * b + c));
            }
        }
    }
    let d1 = Dijkstra::new(n, &g1, 0);
    let d2 = Dijkstra::new(n, &g2, n - 1);
    let mut ans = 1 << 60;
    for i in 0..n {
        ans = ans.min(d1.distance(i) + d2.distance(i));
    }
    println!("{}", ans);
}

pub struct Dijkstra {
    distance: Vec<usize>,
}

impl Dijkstra {
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        const INF: usize = 1 << 60;
        let mut distance = vec![INF; n];
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((std::cmp::Reverse(0), i));
            }
            heap.push((std::cmp::Reverse(INF), i));
        }
        while let Some((std::cmp::Reverse(d), target)) = heap.pop() {
            if distance[target] < d {
                continue;
            }
            distance[target] = d;
            for &(next, cost) in &edges[target] {
                if distance[next] > d + cost {
                    distance[next] = d + cost;
                    heap.push((std::cmp::Reverse(distance[next]), next));
                }
            }
        }
        Self { distance }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }
}
