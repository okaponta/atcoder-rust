use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
       n:usize,
       m:usize,
       st:[(Usize1,Usize1);m],
    }
    let mut edges = vec![vec![]; n];
    for i in 0..m {
        edges[st[i].0].push(st[i].1);
    }
    let d = Dijkstra::new(n, &edges, 0);
    if d.distance(n - 1) == 1 << 60 {
        for _ in 0..m {
            println!("-1");
        }
        return;
    }
    let path = d.get_path(n - 1);
    let mut next = vec![0; n];
    for i in 1..path.len() {
        next[path[i - 1]] = path[i];
    }
    for i in 0..m {
        if next[st[i].0] != st[i].1 {
            println!("{}", d.distance(n - 1));
            continue;
        }
        //計算
        let mut edges = vec![vec![]; n];
        for j in 0..m {
            if i != j {
                edges[st[j].0].push(st[j].1);
            }
        }
        let d = Dijkstra::new(n, &edges, 0);
        let ans = d.distance(n - 1);
        if ans == 1 << 60 {
            println!("-1");
        } else {
            println!("{}", d.distance(n - 1));
        }
    }
}

pub struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    pub fn new(n: usize, edges: &Vec<Vec<usize>>, init: usize) -> Self {
        const INF: usize = 1 << 60;
        let mut distance = vec![INF; n];
        let mut parent = vec![INF; n];
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
        const INF: usize = 1 << 60;
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
