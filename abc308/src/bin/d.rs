use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut g = vec![vec![]; h * w];
    for i in 0..h {
        for j in 0..w {
            for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                let xi = i.wrapping_add(dx);
                let yi = j.wrapping_add(dy);
                if h <= xi || w <= yi {
                    continue;
                }
                // ここは移動可能
                if next(s[i][j]) == s[xi][yi] {
                    g[i * w + j].push(xi * w + yi);
                }
            }
        }
    }
    let d = Dijkstra::new(h * w, &g, 0);
    println!(
        "{}",
        if d.distance(h * w - 1) == 1 << 60 {
            "No"
        } else {
            "Yes"
        }
    );
}

fn next(c: char) -> char {
    match c {
        's' => 'n',
        'n' => 'u',
        'u' => 'k',
        'k' => 'e',
        _ => 's',
    }
}

pub struct Dijkstra {
    distance: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: &Vec<Vec<usize>>, init: usize) -> Self {
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
            for &next in &edges[target] {
                if distance[next] > d + 1 {
                    distance[next] = d + 1;
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
