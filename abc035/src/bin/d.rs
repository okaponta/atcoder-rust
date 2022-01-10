use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,t:usize,
       prize: [usize;n],
       abc:[(Usize1,Usize1,usize);m],
    }
    let mut path = vec![vec![]; n];
    let mut rev_path = vec![vec![]; n];
    for (a, b, c) in abc {
        path[a].push((b, c));
        rev_path[b].push((a, c));
    }
    let d = Dijkstra::new(n, path, 0);
    let rev_d = Dijkstra::new(n, rev_path, 0);
    let mut ans = 0;
    for i in 0..n {
        if d.distance(i) + rev_d.distance(i) >= t {
            continue;
        }
        ans = ans.max(prize[i] * (t - d.distance(i) - rev_d.distance(i)));
    }
    println!("{}", ans);
}

const INF: usize = 1 << 31;

struct Dijkstra {
    distance: Vec<usize>,
    //parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // path: Vec<Vec<(usize,usize)>> path[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, path: Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        // let mut parent = vec![INF; n];
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
            for &(next, cost) in &path[target] {
                if distance[next] > d + cost {
                    distance[next] = d + cost;
                    heap.push((Reverse(distance[next]), next));
                    // parent[next] = target;
                }
            }
        }
        // Self { distance, parent }
        Self { distance }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }

    // !!未テスト!!
    // pub fn get_path(&self, target: usize) -> Vec<usize> {
    //     let mut current = target;
    //     let mut res = vec![current];
    //     while self.parent[current] != INF as usize {
    //         let next = self.parent[current];
    //         res.push(next);
    //         current = next;
    //     }
    //     res.reverse();
    //     res
    // }
}
