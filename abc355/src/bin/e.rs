use proconio::input_interactive;

fn main() {
    input_interactive! {
        n:usize,
        l:usize,
        mut r:usize,
    }
    r += 1;
    let n2 = 1 << n;
    let mut g = vec![vec![]; n2 + 1];
    let mut size = 1;
    for _i in 0..=n {
        for j in (0..n2).step_by(size) {
            g[j].push(j + size);
            g[j + size].push(j);
        }
        size *= 2;
    }
    let d = Dijkstra::new(n2 + 1, &g, l);
    let path = d.get_path(r);
    let mut ans = 1000000;
    for i in 1..path.len() {
        let from = path[i - 1];
        let to = path[i];
        let size = to.max(from) - to.min(from);
        println!("? {} {}", size.trailing_zeros(), to.min(from) / size);
        input_interactive! {t:usize}
        if from < to {
            ans += t;
        } else {
            ans -= t;
        }
    }
    println!("! {}", ans % 100);
}

// 計算量は(E+V)logV
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
