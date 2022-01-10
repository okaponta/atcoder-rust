use std::{cmp::Reverse, collections::BinaryHeap};

const INF: usize = 1 << 31;

struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        let mut parent = vec![INF; n];
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
            for &(next, cost) in &edge[target] {
                if distance[next] > d + cost {
                    distance[next] = d + cost;
                    heap.push((Reverse(distance[next]), next));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkistra() {
        let n = 6;
        let mut path = vec![vec![]; n];
        let abc = vec![
            (0, 1, 5),
            (0, 3, 9),
            (0, 4, 2),
            (1, 2, 2),
            (2, 3, 3),
            (3, 5, 2),
            (4, 5, 3),
        ];
        for (a, b, c) in abc {
            path[a].push((b, c));
            path[b].push((a, c));
        }
        let d = Dijkstra::new(n, path, 0);

        assert_eq!(d.distance, vec![0, 5, 7, 7, 2, 5]);

        assert_eq!(d.get_path(0), vec![0]);
        assert_eq!(d.get_path(1), vec![0, 1]);
        assert_eq!(d.get_path(2), vec![0, 1, 2]);
        assert_eq!(d.get_path(3), vec![0, 4, 5, 3]);
        assert_eq!(d.get_path(4), vec![0, 4]);
        assert_eq!(d.get_path(5), vec![0, 4, 5]);
    }
}
