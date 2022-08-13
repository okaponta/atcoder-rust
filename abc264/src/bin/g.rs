use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        tp:[(Chars,i64);n],
    }
    let node = 27 * 27;
    let mut edges = vec![vec![1 << 60; node]; node];
    let mut one = vec![0; 27];
    let mut two = vec![vec![0; 27]; 27];
    let mut three = vec![vec![0; 27]; node];
    for (t, p) in tp {
        if t.len() == 1 {
            let i = (t[0] as u8 - b'a' + 1) as usize;
            one[i] = -p;
        }
        if t.len() == 2 {
            let i = (t[0] as u8 - b'a' + 1) as usize;
            let j = (t[1] as u8 - b'a' + 1) as usize;
            two[i][j] = -p;
        }
        if t.len() == 3 {
            let i = (t[0] as u8 - b'a' + 1) as usize;
            let j = (t[1] as u8 - b'a' + 1) as usize;
            let k = (t[2] as u8 - b'a' + 1) as usize;
            three[i * 27 + j][k] = -p;
        }
    }
    for i in 0..27 {
        for j in 0..27 {
            for k in 1..27 {
                let from = i * 27 + j;
                let to = j * 27 + k;
                let cost = one[k] + two[j][k] + three[i * 27 + j][k];
                edges[from][to] = cost;
            }
        }
    }
    let mut e = vec![];
    for i in 0..node {
        for j in 0..node {
            if edges[i][j] != 1 << 60 {
                e.push((i, j, edges[i][j]));
            }
        }
    }
    let bf = BellmanFord::new(node, e, 0);
    if bf.has_neg_loop {
        println!("Infinity");
        return;
    }
    let mut ans = 1 << 60;
    for i in 1..node {
        ans = ans.min(bf.distance(i));
    }
    println!("{}", -ans);
}

pub struct BellmanFord {
    distance: Vec<i64>,
    has_neg_loop: bool,
}

impl BellmanFord {
    // n:usize 頂点の数
    // edges: Vec<(usize,usize,i64)> edges[i] = [(0,2,3), (1,3,-1), (From,To,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: Vec<(usize, usize, i64)>, init: usize) -> Self {
        let mut distance = vec![1 << 60; n];
        distance[init] = 0;
        let mut has_neg_loop = false;

        for i in 0..n {
            for edge in &edges {
                let from = edge.0;
                let to = edge.1;
                let cost = edge.2;
                if distance[to] > distance[from] + cost {
                    distance[to] = distance[from] + cost;
                    if i == n - 1 {
                        has_neg_loop = true;
                        break;
                    }
                }
            }
        }
        Self {
            distance,
            has_neg_loop,
        }
    }

    pub fn distance(&self, target: usize) -> i64 {
        self.distance[target]
    }
}
